use once_cell::sync::Lazy;
use ratatui::{
    style::{Color, Style as RatatuiStyle},
    text::Span,
};
use syntect::{
    easy::HighlightLines,
    highlighting::{Style as SyntectStyle, ThemeSet},
    parsing::SyntaxSet,
};

static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(SyntaxSet::load_defaults_newlines);
static THEME_SET: Lazy<ThemeSet> = Lazy::new(|| {
    let mut theme_set = ThemeSet::load_defaults();
    let theme_bytes = include_bytes!("../assets/tokyo-night.tmTheme");
    let mut cursor = std::io::Cursor::new(theme_bytes);
    let theme = ThemeSet::load_from_reader(&mut cursor).expect("Failed to load Tokyo Night theme");
    theme_set.themes.insert("TokyoNight".to_string(), theme);
    theme_set
});

pub fn get_highlighter(lang: &str, theme_name: &str) -> Option<HighlightLines<'static>> {
    let syntax = SYNTAX_SET.find_syntax_by_token(lang)?;
    let theme = THEME_SET.themes.get(theme_name).unwrap_or_else(|| {
        THEME_SET
            .themes
            .get("base16-ocean.dark")
            .unwrap_or_else(|| {
                THEME_SET
                    .themes
                    .values()
                    .next()
                    .expect("No themes available")
            })
    });
    Some(HighlightLines::new(syntax, theme))
}

pub fn get_available_themes() -> Vec<String> {
    let mut themes: Vec<String> = THEME_SET.themes.keys().cloned().collect();
    themes.sort();
    themes
}

pub fn highlight_line(line: &str, highlighter: &mut HighlightLines) -> Vec<Span<'static>> {
    // syntect expects lines to end with \n for many regexes to work
    let line_with_nl = format!("{}\n", line);
    let ranges: Vec<(SyntectStyle, &str)> = highlighter
        .highlight_line(&line_with_nl, &SYNTAX_SET)
        .ok()
        .unwrap_or_default();

    let mut spans: Vec<Span<'static>> = ranges
        .into_iter()
        .map(|(style, text)| {
            let fg = Color::Rgb(style.foreground.r, style.foreground.g, style.foreground.b);
            Span::styled(text.to_string(), RatatuiStyle::default().fg(fg))
        })
        .collect();

    // Remove the trailing newline character from the last span if present
    if let Some(last_span) = spans.last_mut() {
        if last_span.content.ends_with('\n') {
            let new_content = last_span.content.trim_end_matches('\n').to_string();
            if new_content.is_empty() {
                spans.pop();
            } else {
                *last_span = Span::styled(new_content, last_span.style);
            }
        }
    }

    spans
}
