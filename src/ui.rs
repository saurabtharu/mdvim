use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
};

use crate::app::{App, FocusedPane};
use crate::renderer::markdown_to_ratatui;

pub fn render_ui(f: &mut Frame, app: &mut App) {
    let area = f.area();

    let chunks = if app.show_tree {
        let tree = app.tree_width_percentage.min(80).max(10);
        let preview = 100u16.saturating_sub(tree);
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(tree),
                Constraint::Percentage(preview),
            ])
            .split(area)
    } else {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(area)
    };

    // Remember layout sizes for mouse-based resizing.
    app.last_area_width = area.width;
    if app.show_tree {
        app.last_tree_width_px = chunks[0].width;
    } else {
        app.last_tree_width_px = area.width;
    }

    let rendered = markdown_to_ratatui(&app.markdown, &app.current_theme);
    let line_count = rendered.lines.len() as u16;

    let viewport_height = if app.show_tree {
        chunks[1].height.saturating_sub(2)
    } else {
        chunks[0].height.saturating_sub(2)
    };

    app.update_max_scroll(line_count, viewport_height);

    let mut preview_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(255, 182, 193)))
        .title(" 📄 Markdown Preview ")
        .title_style(
            Style::default()
                .fg(Color::Rgb(255, 182, 193))
                .add_modifier(Modifier::BOLD),
        );

    if app.focused_pane == FocusedPane::Preview {
        preview_block = preview_block.border_style(
            Style::default()
                .fg(Color::Rgb(255, 105, 180))
                .add_modifier(Modifier::BOLD),
        );
    }

    let preview = Paragraph::new(rendered)
        .block(preview_block)
        .scroll((app.scroll_offset, 0))
        .wrap(ratatui::widgets::Wrap { trim: false });

    if app.show_tree {
        let items: Vec<ListItem> = app
            .files
            .iter()
            .map(|p| {
                let name = p.file_name().unwrap_or_default().to_string_lossy();
                let icon = if p.is_dir() { "📁 " } else { "📄 " };
                ListItem::new(format!("{}{}", icon, name))
            })
            .collect();

        let mut tree_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(135, 206, 250)))
            .title(" 📂 Files ")
            .title_style(
                Style::default()
                    .fg(Color::Rgb(135, 206, 250))
                    .add_modifier(Modifier::BOLD),
            );

        if app.focused_pane == FocusedPane::FileTree {
            tree_block = tree_block.border_style(
                Style::default()
                    .fg(Color::Rgb(255, 105, 180))
                    .add_modifier(Modifier::BOLD),
            );
        }

        let list = List::new(items)
            .block(tree_block)
            .highlight_style(
                Style::default()
                    .fg(Color::Rgb(255, 105, 180))
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▶ ");

        let mut state = ListState::default();
        state.select(Some(app.selected));

        f.render_stateful_widget(list, chunks[0], &mut state);
        f.render_widget(preview, chunks[1]);
    } else {
        f.render_widget(preview, chunks[0]);
    }

    // Render theme selection popup
    if app.show_theme_list {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Select Theme ")
            .border_style(Style::default().fg(Color::Yellow));

        let area = centered_rect(60, 40, f.area());
        f.render_widget(Clear, area); // Clear background

        let items: Vec<ListItem> = app
            .available_themes
            .iter()
            .map(|t| {
                let style = if t == &app.current_theme {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(t.as_str()).style(style)
            })
            .collect();

        let list = List::new(items).block(block).highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

        f.render_stateful_widget(list, area, &mut app.theme_list_state);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
