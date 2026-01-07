use pulldown_cmark::{
    Alignment, CodeBlockKind, Event as MdEvent, HeadingLevel, Options, Parser, Tag, TagEnd,
};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
};

use crate::syntax::{get_highlighter, highlight_line};
use syntect::easy::HighlightLines;

pub fn markdown_to_ratatui(md: &str) -> Text<'static> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_DEFINITION_LIST);
    options.insert(Options::ENABLE_SUPERSCRIPT);
    options.insert(Options::ENABLE_SUBSCRIPT);
    options.insert(Options::ENABLE_WIKILINKS);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_GFM);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_MATH);
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);

    let parser = Parser::new_ext(md, options);

    let mut lines: Vec<Line> = Vec::new();
    let mut spans: Vec<Span<'static>> = Vec::new();

    let mut bold = false;
    let mut italic = false;
    let mut strikethrough = false;
    let mut superscript = false;
    let mut subscript = false;
    let mut in_heading = false;
    let mut heading_level = 0;
    let mut in_code_block = false;
    let mut code_block_lang = String::new();
    let mut in_list = false;
    let mut list_depth: usize = 0;
    let mut link_url = String::new();
    let mut current_highlighter: Option<HighlightLines<'static>> = None;

    // Table state
    let mut in_table = false;
    let mut table_alignments: Vec<Alignment> = Vec::new();
    let mut table_headers: Vec<Vec<Span<'static>>> = Vec::new();
    let mut table_rows: Vec<Vec<Vec<Span<'static>>>> = Vec::new();
    let mut current_row: Vec<Vec<Span<'static>>> = Vec::new();
    let mut current_cell: Vec<Span<'static>> = Vec::new();
    let mut is_header_row = false;

    for event in parser {
        match event {
            MdEvent::Start(Tag::Strong) => bold = true,
            MdEvent::End(TagEnd::Strong) => bold = false,

            MdEvent::Start(Tag::Emphasis) => italic = true,
            MdEvent::End(TagEnd::Emphasis) => italic = false,

            MdEvent::Start(Tag::Strikethrough) => {
                strikethrough = true;
            }
            MdEvent::End(TagEnd::Strikethrough) => {
                strikethrough = false;
            }

            MdEvent::Start(Tag::Superscript) => superscript = true,
            MdEvent::End(TagEnd::Superscript) => superscript = false,

            MdEvent::Start(Tag::Subscript) => subscript = true,
            MdEvent::End(TagEnd::Subscript) => subscript = false,

            // Headings
            MdEvent::Start(Tag::Heading { level, .. }) => {
                in_heading = true;
                heading_level = match level {
                    HeadingLevel::H1 => 1,
                    HeadingLevel::H2 => 2,
                    HeadingLevel::H3 => 3,
                    HeadingLevel::H4 => 4,
                    HeadingLevel::H5 => 5,
                    HeadingLevel::H6 => 6,
                };

                if heading_level == 1 {
                    spans.push(Span::raw("\n"));
                }

                // Add # symbols for H2-H6
                if heading_level >= 2 {
                    let symbol_color = match heading_level {
                        2 => Color::Rgb(135, 206, 250),
                        3 => Color::Rgb(255, 182, 193),
                        4 => Color::Rgb(144, 238, 144),
                        5 => Color::Rgb(221, 160, 221),
                        _ => Color::Rgb(255, 218, 185),
                    };

                    spans.push(Span::styled(
                        format!("{} ", "#".repeat(heading_level)),
                        Style::default()
                            .fg(symbol_color)
                            .add_modifier(Modifier::BOLD),
                    ));
                }
            }

            MdEvent::End(TagEnd::Heading(_)) => {
                in_heading = false;
                if !spans.is_empty() {
                    lines.push(Line::from(spans.drain(..).collect::<Vec<_>>()));

                    if heading_level == 1 {
                        lines.push(Line::from(vec![Span::styled(
                            "━".repeat(70),
                            Style::default().fg(Color::Rgb(135, 206, 250)),
                        )]));
                    }

                    lines.push(Line::default());
                }
            }

            // Code blocks
            MdEvent::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                lines.push(Line::default());

                code_block_lang = match kind {
                    CodeBlockKind::Fenced(lang) => {
                        let lang_str = lang.to_string();
                        let actual_lang = if !lang_str.is_empty() {
                            lines.push(Line::from(vec![
                                Span::styled(" ┌─ ", Style::default().fg(Color::DarkGray)),
                                Span::styled(
                                    lang_str.clone(),
                                    Style::default()
                                        .fg(Color::Rgb(255, 182, 193))
                                        .add_modifier(Modifier::ITALIC),
                                ),
                            ]));
                            lang_str
                        } else {
                            lines.push(Line::from(vec![Span::styled(
                                " ┌─────",
                                Style::default().fg(Color::DarkGray),
                            )]));
                            String::new()
                        };

                        // Initialize highlighter if we have a language
                        if !actual_lang.is_empty() {
                            current_highlighter = get_highlighter(&actual_lang);
                        }
                        actual_lang
                    }
                    CodeBlockKind::Indented => {
                        lines.push(Line::from(vec![Span::styled(
                            " ┌─────",
                            Style::default().fg(Color::DarkGray),
                        )]));
                        String::new()
                    }
                };
            }

            MdEvent::End(TagEnd::CodeBlock) => {
                in_code_block = false;
                code_block_lang.clear();
                current_highlighter = None;
                lines.push(Line::from(vec![Span::styled(
                    " └─────",
                    Style::default().fg(Color::DarkGray),
                )]));
                lines.push(Line::default());
            }

            // Lists
            MdEvent::Start(Tag::List(_)) => {
                in_list = true;
                list_depth += 1;
                if list_depth == 1 {
                    lines.push(Line::default());
                }
            }

            MdEvent::End(TagEnd::List(_)) => {
                list_depth -= 1;
                if list_depth == 0 {
                    in_list = false;
                    lines.push(Line::default());
                }
            }

            MdEvent::Start(Tag::Item) => {}

            MdEvent::End(TagEnd::Item) => {
                if !spans.is_empty() {
                    let indent = "  ".repeat(list_depth.saturating_sub(1));
                    let bullet_colors = [
                        Color::Rgb(255, 182, 193),
                        Color::Rgb(173, 216, 230),
                        Color::Rgb(255, 218, 185),
                    ];
                    let color = bullet_colors[(list_depth - 1) % bullet_colors.len()];

                    let mut item_spans = vec![
                        Span::raw(indent),
                        Span::styled(" • ", Style::default().fg(color)),
                    ];
                    item_spans.extend(spans.drain(..));
                    lines.push(Line::from(item_spans));
                }
            }

            // Task list item
            MdEvent::TaskListMarker(checked) => {
                let checkbox = if checked {
                    Span::styled("[✓] ", Style::default().fg(Color::LightGreen))
                } else {
                    Span::styled("[ ] ", Style::default().fg(Color::DarkGray))
                };
                spans.insert(0, checkbox);
            }

            // Blockquote
            MdEvent::Start(Tag::BlockQuote(_)) => {
                lines.push(Line::default());
            }

            MdEvent::End(TagEnd::BlockQuote(_)) => {
                if !spans.is_empty() {
                    let quote_text: Vec<Span> = spans.drain(..).collect();
                    let mut quote_line = vec![Span::styled(
                        " ┃ ",
                        Style::default()
                            .fg(Color::Rgb(255, 182, 193))
                            .add_modifier(Modifier::BOLD),
                    )];
                    quote_line.extend(quote_text);
                    lines.push(Line::from(quote_line));
                }
                lines.push(Line::default());
            }

            // Links
            MdEvent::Start(Tag::Link { dest_url, .. }) => {
                link_url = dest_url.to_string();
            }

            MdEvent::End(TagEnd::Link) => {
                if !link_url.is_empty() {
                    let link_span = Span::styled(
                        format!(" (🔗 {})", link_url),
                        Style::default()
                            .fg(Color::Rgb(135, 206, 250))
                            .add_modifier(Modifier::DIM),
                    );
                    if in_table {
                        current_cell.push(link_span);
                    } else {
                        spans.push(link_span);
                    }
                    link_url.clear();
                }
            }

            // Images
            MdEvent::Start(Tag::Image { dest_url, .. }) => {
                let image_span =
                    Span::styled("🖼️  ", Style::default().fg(Color::Rgb(255, 182, 193)));
                if in_table {
                    current_cell.push(image_span);
                } else {
                    spans.push(image_span);
                }
                link_url = dest_url.to_string();
            }

            MdEvent::End(TagEnd::Image) => {
                if !link_url.is_empty() {
                    let url_span = Span::styled(
                        format!(" ({})", link_url),
                        Style::default()
                            .fg(Color::Rgb(135, 206, 250))
                            .add_modifier(Modifier::DIM),
                    );
                    if in_table {
                        current_cell.push(url_span);
                    } else {
                        spans.push(url_span);
                    }
                    link_url.clear();
                }
            }

            MdEvent::End(TagEnd::Paragraph) => {
                if !spans.is_empty() && !in_list && !in_table {
                    lines.push(Line::from(spans.drain(..).collect::<Vec<_>>()));
                    lines.push(Line::default());
                }
            }

            // Text
            MdEvent::Text(text) => {
                if in_code_block {
                    for line in text.split('\n') {
                        if !spans.is_empty() {
                            lines.push(Line::from(spans.drain(..).collect::<Vec<_>>()));
                        }

                        let mut line_spans =
                            vec![Span::styled(" │ ", Style::default().fg(Color::DarkGray))];

                        if let Some(highlighter) = &mut current_highlighter {
                            line_spans.extend(highlight_line(line, highlighter));
                        } else {
                            line_spans.push(Span::styled(
                                line.to_string(),
                                Style::default().fg(Color::White),
                            ));
                        }

                        lines.push(Line::from(line_spans));
                    }
                } else {
                    let mut style = Style::default().fg(Color::White);

                    if in_heading && heading_level == 1 {
                        style = Style::default()
                            .fg(Color::White)
                            .bg(Color::Rgb(65, 105, 225))
                            .add_modifier(Modifier::BOLD);
                    } else if in_heading {
                        let color = match heading_level {
                            2 => Color::Rgb(135, 206, 250),
                            3 => Color::Rgb(255, 182, 193),
                            4 => Color::Rgb(144, 238, 144),
                            5 => Color::Rgb(221, 160, 221),
                            _ => Color::Rgb(255, 218, 185),
                        };
                        style = style.fg(color).add_modifier(Modifier::BOLD);
                    } else {
                        // Build modifiers and colors based on active formatting
                        let mut modifiers = Modifier::empty();
                        let mut text_color = Color::White;

                        // Apply strikethrough modifier (but don't override color yet)
                        if strikethrough {
                            modifiers |= Modifier::CROSSED_OUT;
                        }

                        // Apply bold/italic colors (strikethrough can coexist)
                        if bold {
                            modifiers |= Modifier::BOLD;
                            if !strikethrough {
                                text_color = Color::Rgb(255, 218, 185);
                            } else {
                                // When strikethrough is active, use a muted version of bold color
                                text_color = Color::Rgb(169, 169, 169);
                            }
                        }
                        if italic {
                            modifiers |= Modifier::ITALIC;
                            if !bold {
                                if !strikethrough {
                                    text_color = Color::Rgb(221, 160, 221);
                                } else {
                                    // When strikethrough is active, use gray
                                    text_color = Color::Rgb(169, 169, 169);
                                }
                            }
                        }

                        // If only strikethrough (no bold/italic), use gray
                        if strikethrough && !bold && !italic {
                            text_color = Color::Rgb(169, 169, 169);
                        }

                        if superscript {
                            modifiers |= Modifier::DIM;
                            if !strikethrough && !bold && !italic {
                                text_color = Color::LightCyan;
                            }
                        }
                        if subscript {
                            modifiers |= Modifier::DIM;
                            if !strikethrough && !bold && !italic && !superscript {
                                text_color = Color::LightMagenta;
                            }
                        }

                        style = style.fg(text_color).add_modifier(modifiers);
                    }

                    // Format text for superscript/subscript display
                    let display_text = if superscript && !in_heading {
                        format!("^{}", text)
                    } else if subscript && !in_heading {
                        format!("_{}", text)
                    } else {
                        text.to_string()
                    };

                    let text_span = Span::styled(display_text, style);
                    if in_table {
                        current_cell.push(text_span);
                    } else {
                        spans.push(text_span);
                    }
                }
            }

            // Inline code
            MdEvent::Code(code) => {
                let code_span = Span::styled(
                    format!(" {} ", code),
                    Style::default()
                        .fg(Color::Rgb(220, 80, 80))
                        .bg(Color::Rgb(60, 60, 60))
                        .add_modifier(Modifier::BOLD),
                );
                if in_table {
                    current_cell.push(code_span);
                } else {
                    spans.push(code_span);
                }
            }

            // Math
            MdEvent::InlineMath(text) => {
                let math_span = Span::styled(
                    format!(" ${}$ ", text),
                    Style::default()
                        .fg(Color::Rgb(144, 238, 144))
                        .add_modifier(Modifier::ITALIC),
                );
                if in_table {
                    current_cell.push(math_span);
                } else {
                    spans.push(math_span);
                }
            }
            MdEvent::DisplayMath(text) => {
                if !spans.is_empty() {
                    lines.push(Line::from(spans.drain(..).collect::<Vec<_>>()));
                }
                lines.push(Line::default());
                lines.push(Line::from(Span::styled(
                    format!(" $${}$$ ", text),
                    Style::default()
                        .fg(Color::Rgb(144, 238, 144))
                        .add_modifier(Modifier::ITALIC),
                )));
                lines.push(Line::default());
            }

            // Footnotes (basic handling)
            MdEvent::FootnoteReference(label) => {
                let foot_span = Span::styled(
                    format!("[^{}]", label),
                    Style::default()
                        .fg(Color::LightCyan)
                        .add_modifier(Modifier::DIM),
                );
                if in_table {
                    current_cell.push(foot_span);
                } else {
                    spans.push(foot_span);
                }
            }

            MdEvent::SoftBreak => {
                let space = Span::raw(" ");
                if in_table {
                    current_cell.push(space);
                } else {
                    spans.push(space);
                }
            }

            MdEvent::HardBreak => {
                if in_table {
                    current_cell.push(Span::raw(" "));
                } else if !spans.is_empty() {
                    lines.push(Line::from(spans.drain(..).collect::<Vec<_>>()));
                }
            }

            MdEvent::Rule => {
                lines.push(Line::default());
                lines.push(Line::from(vec![Span::styled(
                    "─".repeat(70),
                    Style::default().fg(Color::Rgb(255, 182, 193)),
                )]));
                lines.push(Line::default());
            }

            // Tables
            MdEvent::Start(Tag::Table(alignments)) => {
                in_table = true;
                table_alignments = alignments;
                table_headers.clear();
                table_rows.clear();
                current_row.clear();
                current_cell.clear();
                is_header_row = false;
            }

            MdEvent::End(TagEnd::Table) => {
                in_table = false;
                render_table(&mut lines, &table_headers, &table_rows, &table_alignments);
            }

            MdEvent::Start(Tag::TableHead) => {
                is_header_row = true;
            }

            MdEvent::End(TagEnd::TableHead) => {
                // If we're ending the header section and haven't stored headers yet, store them now
                // This handles the case where headers might not have been stored in End(TableRow)
                if is_header_row && table_headers.is_empty() && !current_row.is_empty() {
                    table_headers = std::mem::take(&mut current_row);
                }
                is_header_row = false;
            }

            MdEvent::Start(Tag::TableRow) => {
                current_row.clear();
            }

            MdEvent::End(TagEnd::TableRow) => {
                if is_header_row {
                    // Always store header row - in standard markdown there's only one header row
                    // Even if cells are empty, we need to store the structure
                    table_headers = std::mem::take(&mut current_row);
                } else if !current_row.is_empty() {
                    table_rows.push(std::mem::take(&mut current_row));
                }
            }

            MdEvent::Start(Tag::TableCell) => {
                current_cell.clear();
            }

            MdEvent::End(TagEnd::TableCell) => {
                let trimmed_cell = trim_spans(std::mem::take(&mut current_cell));
                current_row.push(trimmed_cell);
            }

            _ => {}
        }
    }

    if !spans.is_empty() {
        lines.push(Line::from(spans));
    }

    Text::from(lines)
}

fn render_table(
    lines: &mut Vec<Line<'static>>,
    headers: &[Vec<Span<'static>>],
    rows: &[Vec<Vec<Span<'static>>>],
    alignments: &[Alignment],
) {
    if headers.is_empty() && rows.is_empty() {
        return;
    }

    let col_count = headers
        .len()
        .max(rows.iter().map(|r| r.len()).max().unwrap_or(0));

    if col_count == 0 {
        return;
    }

    // Calculate column widths based on content length (approximating display width with len for simplicity)
    let mut col_widths: Vec<usize> = vec![5; col_count];

    // Headers
    for (i, cell) in headers.iter().enumerate() {
        let cell_width: usize = cell.iter().map(|s| s.content.len()).sum();
        col_widths[i] = col_widths[i].max(cell_width);
    }

    // Rows
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            let cell_width: usize = cell.iter().map(|s| s.content.len()).sum();
            if i < col_widths.len() {
                col_widths[i] = col_widths[i].max(cell_width);
            }
        }
    }

    // Add padding (2 spaces on each side)
    for width in &mut col_widths {
        *width += 4;
    }

    let border_color = Color::Rgb(135, 206, 250);
    lines.push(Line::default());

    // Always render headers if we have a column count (headers should always exist in markdown tables)
    if col_count > 0 {
        let mut header_line: Vec<Span<'static>> = vec![Span::raw("  ")];
        let header_style = Style::default()
            .fg(Color::Rgb(255, 182, 193))
            .add_modifier(Modifier::BOLD);

        for i in 0..col_count {
            let cell = headers.get(i).cloned().unwrap_or_default();
            let mut formatted = format_cell_spans(
                &cell,
                col_widths[i],
                alignments.get(i).unwrap_or(&Alignment::None),
            );
            for span in &mut formatted {
                span.style = span.style.patch(header_style);
            }
            header_line.extend(formatted);

            if i < col_count - 1 {
                header_line.push(Span::styled(" │ ", Style::default().fg(border_color)));
            }
        }
        lines.push(Line::from(header_line));

        // Separator - must match header line indentation
        let mut sep: Vec<Span<'static>> = vec![Span::raw("  ")];
        for (i, &width) in col_widths.iter().enumerate() {
            sep.push(Span::styled(
                "─".repeat(width),
                Style::default().fg(border_color),
            ));
            if i < col_widths.len() - 1 {
                sep.push(Span::styled("─┼─", Style::default().fg(border_color)));
            }
        }
        lines.push(Line::from(sep));
    }

    // Rows - must match header line indentation
    for row in rows {
        let mut row_line: Vec<Span<'static>> = vec![Span::raw("  ")];
        for i in 0..col_count {
            let cell = row.get(i).cloned().unwrap_or_default();
            let formatted = format_cell_spans(
                &cell,
                col_widths[i],
                alignments.get(i).unwrap_or(&Alignment::None),
            );
            row_line.extend(formatted);

            if i < col_count - 1 {
                row_line.push(Span::styled(" │ ", Style::default().fg(border_color)));
            }
        }
        lines.push(Line::from(row_line));
    }

    lines.push(Line::default());
}

fn format_cell_spans(
    cell_spans: &[Span<'static>],
    width: usize,
    alignment: &Alignment,
) -> Vec<Span<'static>> {
    let current_width: usize = cell_spans.iter().map(|s| s.content.len()).sum();
    if current_width >= width {
        return cell_spans.to_vec();
    }

    let padding = width - current_width;
    match alignment {
        Alignment::Left | Alignment::None => {
            let mut v = cell_spans.to_vec();
            v.push(Span::raw(" ".repeat(padding)));
            v
        }
        Alignment::Right => {
            let mut v = vec![Span::raw(" ".repeat(padding))];
            v.extend_from_slice(cell_spans);
            v
        }
        Alignment::Center => {
            let left_pad = padding / 2;
            let right_pad = padding - left_pad;
            let mut v = vec![Span::raw(" ".repeat(left_pad))];
            v.extend_from_slice(cell_spans);
            v.push(Span::raw(" ".repeat(right_pad)));
            v
        }
    }
}

fn trim_spans(mut spans: Vec<Span<'static>>) -> Vec<Span<'static>> {
    // Trim leading whitespace
    while !spans.is_empty() {
        let trimmed = spans[0].content.trim_start();
        if trimmed.len() == spans[0].content.len() {
            break;
        }
        if trimmed.is_empty() {
            spans.remove(0);
        } else {
            let style = spans[0].style;
            spans[0] = Span::styled(trimmed.to_string(), style);
            break;
        }
    }

    // Trim trailing whitespace
    while !spans.is_empty() {
        let last_idx = spans.len() - 1;
        let trimmed = spans[last_idx].content.trim_end();
        if trimmed.len() == spans[last_idx].content.len() {
            break;
        }
        if trimmed.is_empty() {
            spans.pop();
        } else {
            let style = spans[last_idx].style;
            spans[last_idx] = Span::styled(trimmed.to_string(), style);
            break;
        }
    }

    spans
}
