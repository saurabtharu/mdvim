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
        "bash" | "sh" | "shell" => {
            if matches!(
                token,
                "if" | "then" | "else" | "elif" | "fi" | "for" | "while" | "do" | "done"
                    | "case" | "esac" | "function" | "return" | "export" | "local" | "readonly"
                    | "echo" | "printf" | "test" | "[" | "]" | "true" | "false"
            ) {
                Color::LightMagenta
            } else if token.starts_with('"') || token.starts_with('\'') || token.starts_with('$') {
                Color::LightGreen
            } else if token.starts_with('#') {
                Color::DarkGray
            } else if token.chars().all(|c| c.is_numeric() || c == '.') {
                Color::LightCyan
            } else {
                Color::White
            }
        }
        "yaml" | "yml" => {
            if token.starts_with('#') {
                Color::DarkGray
            } else if token.starts_with('"') || token.starts_with('\'') {
                Color::LightGreen
            } else if token.ends_with(':') {
                Color::LightMagenta
            } else {
                Color::White
            }
        }
        "json" => {
            if token.starts_with('"') {
                Color::LightGreen
            } else if matches!(token, "true" | "false" | "null") {
                Color::LightMagenta
            } else if token.chars().all(|c| c.is_numeric() || c == '.' || c == '-' || c == 'e' || c == 'E') {
                Color::LightCyan
            } else {
                Color::White
            }
        }
        "html" | "xml" => {
            if token.starts_with('<') || token.starts_with("</") {
                Color::LightMagenta
            } else if token.starts_with('"') || token.starts_with('\'') {
                Color::LightGreen
            } else {
                Color::White
            }
        }
        "css" => {
            if token.ends_with(':') || token.ends_with(';') {
                Color::LightMagenta
            } else if token.starts_with('#') || token.starts_with('.') {
                Color::LightCyan
            } else if token.starts_with('"') || token.starts_with('\'') {
                Color::LightGreen
            } else {
                Color::White
            }
        }
        "sql" => {
            if matches!(
                token,
                "SELECT" | "FROM" | "WHERE" | "INSERT" | "UPDATE" | "DELETE" | "CREATE"
                    | "DROP" | "ALTER" | "TABLE" | "INDEX" | "VIEW" | "JOIN" | "INNER"
                    | "LEFT" | "RIGHT" | "OUTER" | "ON" | "GROUP" | "BY" | "ORDER"
                    | "HAVING" | "AS" | "AND" | "OR" | "NOT" | "NULL" | "IS" | "IN"
                    | "LIKE" | "BETWEEN" | "EXISTS" | "UNION" | "DISTINCT" | "LIMIT"
            ) {
                Color::LightMagenta
            } else if token.starts_with('"') || token.starts_with('\'') {
                Color::LightGreen
            } else if token.starts_with("--") || token.starts_with("/*") {
                Color::DarkGray
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
    let mut in_comment = false;

    // Determine comment style based on language
    let comment_style = match lang {
        "bash" | "sh" | "shell" | "python" | "py" | "yaml" | "yml" => "#",
        _ => "//", // Default to // for most languages
    };

    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];
        
        if in_comment {
            // Everything after comment marker is grey
            let comment_text: String = chars[i..].iter().collect();
            spans.push(Span::styled(
                comment_text,
                Style::default().fg(Color::DarkGray),
            ));
            break; // Rest of line is comment
        }

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
        } else if comment_style == "//" && i + 1 < chars.len() && ch == '/' && chars[i + 1] == '/' {
            // Found // comment
            if !current_token.is_empty() {
                let color = get_syntax_color(&current_token, lang);
                spans.push(Span::styled(
                    current_token.clone(),
                    Style::default().fg(color),
                ));
                current_token.clear();
            }
            in_comment = true;
            // Include the // in the comment
            let comment_text: String = chars[i..].iter().collect();
            spans.push(Span::styled(
                comment_text,
                Style::default().fg(Color::DarkGray),
            ));
            break;
        } else if comment_style == "#" && ch == '#' {
            // Found # comment
            if !current_token.is_empty() {
                let color = get_syntax_color(&current_token, lang);
                spans.push(Span::styled(
                    current_token.clone(),
                    Style::default().fg(color),
                ));
                current_token.clear();
            }
            in_comment = true;
            // Include the # and rest of line in the comment
            let comment_text: String = chars[i..].iter().collect();
            spans.push(Span::styled(
                comment_text,
                Style::default().fg(Color::DarkGray),
            ));
            break;
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
        
        i += 1;
    }

    // Handle remaining token if not in comment
    if !in_comment && !in_string && !current_token.is_empty() {
        let color = get_syntax_color(&current_token, lang);
        spans.push(Span::styled(current_token, Style::default().fg(color)));
    } else if !in_comment && in_string && !current_token.is_empty() {
        spans.push(Span::styled(
            current_token,
            Style::default().fg(Color::LightGreen),
        ));
    }

    spans
}
