/// # Converter
/// Converts markup to HTML
pub struct Converter {
    in_code_block: bool,
}

impl Converter {
    pub fn new() -> Self {
        Converter {
            in_code_block: false,
        }
    }

    /// Converts markup to HTML
    /// # Arguments
    /// * `markup` - The markup to convert
    /// # Returns
    /// * `String` - The HTML
    /// # Examples
    /// ```
    /// use markup::converter::Converter;
    /// let mut converter = Converter::new();
    /// let html = converter.convert_to_html("@ Title");
    /// assert_eq!(html, "<h1>Title</h1>");
    /// ```
    pub fn convert_to_html(&mut self, markup: &str) -> String {
        let mut html = String::new();
        let mut in_list = false;

        for line in markup.lines() {
            if line.starts_with("@") {
                html.push_str(&format!("<h1>{}</h1>", &line[2..]));
            } else if line.starts_with("**") && line.ends_with("**") {
                html.push_str(&format!("<strong>{}</strong>", &line[2..line.len() - 2]));
            } else if line.starts_with("^") && line.ends_with("^") {
                html.push_str(&format!("<em>{}</em>", &line[1..line.len() - 1]));
            } else if line.starts_with("\\") && line.ends_with("\\") {
                html.push_str(&format!("<code>{}</code>", &line[1..line.len() - 1]));
            } else if line.starts_with("\\\\\\") && line.ends_with("\\\\\\") {
                if self.in_code_block {
                    html.push_str("</pre>");
                    self.in_code_block = false;
                } else {
                    html.push_str("<pre>");
                    self.in_code_block = true;
                }
            } else if line.starts_with("~~") && line.ends_with("~~Link~~") {
                let parts: Vec<&str> = line.split("~~").collect();
                if let [text, _, link, _] = parts[..] {
                    html.push_str(&format!("<a href='{}'>{}</a>", link, text));
                }
            } else if line.starts_with("---") {
                html.push_str("<hr>");
            } else if line.starts_with(">") {
                if !in_list {
                    html.push_str("<ul>");
                    in_list = true;
                }
                html.push_str(&format!("<li>{}</li>", &line[2..]));

            } else {
                if in_list {
                    html.push_str("</ul>");
                    in_list = false;
                }
                html.push_str(&format!("{}<br>", line));
            }
        }

        if self.in_code_block {
            html.push_str("</pre>");
        }
        if in_list {
            html.push_str("</ul>");
        }

        html
    }
}
