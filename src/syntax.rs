use ratatui::{
    style::{Color, Style},
    text::Span,
};

pub fn get_syntax_color(token: &str, lang: &str) -> Color {
    match lang {
        "rust" | "rs" => {
            if matches!(
                token,
                "fn" | "let"
                    | "mut"
                    | "const"
                    | "static"
                    | "struct"
                    | "enum"
                    | "impl"
                    | "trait"
                    | "type"
                    | "use"
                    | "mod"
                    | "pub"
                    | "crate"
                    | "self"
                    | "super"
                    | "if"
                    | "else"
                    | "match"
                    | "loop"
                    | "while"
                    | "for"
                    | "in"
                    | "return"
                    | "break"
                    | "continue"
                    | "async"
                    | "await"
                    | "move"
                    | "ref"
                    | "unsafe"
                    | "where"
                    | "as"
                    | "dyn"
                    | "Box"
                    | "Vec"
                    | "String"
                    | "Option"
                    | "Result"
            ) {
                Color::LightMagenta
            } else if token.starts_with('"') || token.starts_with('\'') {
                Color::LightGreen
            } else if token.starts_with("//") || token.starts_with("/*") {
                Color::DarkGray
            } else if token.chars().all(|c| c.is_numeric() || c == '.') {
                Color::LightCyan
            } else {
                Color::White
            }
        }
        "javascript" | "js" | "typescript" | "ts" => {
            if matches!(
                token,
                "const"
                    | "let"
                    | "var"
                    | "function"
                    | "async"
                    | "await"
                    | "return"
                    | "if"
                    | "else"
                    | "for"
                    | "while"
                    | "do"
                    | "switch"
                    | "case"
                    | "break"
                    | "continue"
                    | "try"
                    | "catch"
                    | "finally"
                    | "throw"
                    | "new"
                    | "class"
                    | "extends"
                    | "import"
                    | "export"
                    | "from"
                    | "default"
                    | "this"
                    | "super"
                    | "typeof"
            ) {
                Color::LightMagenta
            } else if token.starts_with('"') || token.starts_with('\'') || token.starts_with('`') {
                Color::LightGreen
            } else if token.starts_with("//") || token.starts_with("/*") {
                Color::DarkGray
            } else if token.chars().all(|c| c.is_numeric() || c == '.') {
                Color::LightCyan
            } else {
                Color::White
            }
        }
        "python" | "py" => {
            if matches!(
                token,
                "def"
                    | "class"
                    | "return"
                    | "if"
                    | "elif"
                    | "else"
                    | "for"
                    | "while"
                    | "in"
                    | "is"
                    | "not"
                    | "and"
                    | "or"
                    | "True"
                    | "False"
                    | "None"
                    | "try"
                    | "except"
                    | "finally"
                    | "raise"
                    | "import"
                    | "from"
                    | "as"
                    | "with"
                    | "pass"
                    | "break"
            ) {
                Color::LightMagenta
            } else if token.starts_with('"') || token.starts_with('\'') {
                Color::LightGreen
            } else if token.starts_with('#') {
                Color::DarkGray
            } else if token.chars().all(|c| c.is_numeric() || c == '.') {
                Color::LightCyan
            } else {
                Color::White
            }
        }
        "go" => {
            if matches!(
                token,
                "func"
                    | "var"
                    | "const"
                    | "type"
                    | "struct"
                    | "interface"
                    | "package"
                    | "import"
                    | "return"
                    | "if"
                    | "else"
                    | "for"
                    | "range"
                    | "switch"
                    | "case"
                    | "default"
                    | "go"
                    | "defer"
                    | "select"
                    | "chan"
                    | "map"
                    | "make"
                    | "new"
                    | "nil"
                    | "true"
                    | "false"
                    | "break"
                    | "continue"
            ) {
                Color::LightMagenta
            } else if token.starts_with('"') || token.starts_with('`') {
                Color::LightGreen
            } else if token.starts_with("//") || token.starts_with("/*") {
                Color::DarkGray
            } else if token.chars().all(|c| c.is_numeric() || c == '.') {
                Color::LightCyan
            } else {
                Color::White
            }
        }
        "java" | "c" | "cpp" | "c++" => {
            if matches!(
                token,
                "class"
                    | "public"
                    | "private"
                    | "protected"
                    | "static"
                    | "void"
                    | "int"
                    | "string"
                    | "float"
                    | "double"
                    | "boolean"
                    | "if"
                    | "else"
                    | "for"
                    | "while"
                    | "return"
                    | "new"
                    | "this"
                    | "super"
                    | "extends"
                    | "implements"
                    | "true"
                    | "false"
                    | "null"
                    | "import"
                    | "package"
            ) {
                Color::LightMagenta
            } else if token.starts_with('"') || token.starts_with('\'') {
                Color::LightGreen
            } else if token.starts_with("//") || token.starts_with("/*") {
                Color::DarkGray
            } else if token.chars().all(|c| c.is_numeric() || c == '.') {
                Color::LightCyan
            } else {
                Color::White
            }
        }
        _ => Color::White,
    }
}

pub fn highlight_code_line(line: &str, lang: &str) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    let mut current_token = String::new();
    let mut in_string = false;
    let mut string_char = ' ';

    for ch in line.chars() {
        if in_string {
            current_token.push(ch);
            if ch == string_char
                && !current_token.ends_with("\\\"")
                && !current_token.ends_with("\\'")
            {
                spans.push(Span::styled(
                    current_token.clone(),
                    Style::default().fg(Color::LightGreen),
                ));
                current_token.clear();
                in_string = false;
            }
        } else if ch == '"' || ch == '\'' || ch == '`' {
            if !current_token.is_empty() {
                let color = get_syntax_color(&current_token, lang);
                spans.push(Span::styled(
                    current_token.clone(),
                    Style::default().fg(color),
                ));
                current_token.clear();
            }
            in_string = true;
            string_char = ch;
            current_token.push(ch);
        } else if ch.is_whitespace() || "(){}[];,:".contains(ch) {
            if !current_token.is_empty() {
                let color = get_syntax_color(&current_token, lang);
                spans.push(Span::styled(
                    current_token.clone(),
                    Style::default().fg(color),
                ));
                current_token.clear();
            }
            spans.push(Span::raw(ch.to_string()));
        } else {
            current_token.push(ch);
        }
    }

    if in_string || !current_token.is_empty() {
        let color = if in_string {
            Color::LightGreen
        } else {
            get_syntax_color(&current_token, lang)
        };
        spans.push(Span::styled(current_token, Style::default().fg(color)));
    }

    spans
}
