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
    /// use darkdown::converter::converter::Converter;
    /// let mut converter = Converter::new();
    /// let html = converter.convert_to_html("@ Title");
    /// assert_eq!(html, "<h1>Title</h1>");
    /// let html = converter.convert_to_html("**Bold**");
    /// assert_eq!(html, "<strong>Bold</strong>");
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
            } else if line.starts_with("~~") && line.ends_with("~~") {
                let parts: Vec<&str> = line.split("~~").collect();
                if let [_, text, link, _] = parts[..] {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title() {
        let mut converter = Converter::new();
        let html = converter.convert_to_html("@ Title");
        assert_eq!(html, "<h1>Title</h1>");
    }
    #[test]
    fn test_bold_italic() {
        let mut converter = Converter::new();
        let html = converter.convert_to_html("**Bold**");
        assert_eq!(html, "<strong>Bold</strong>");
        let html = converter.convert_to_html("^Italic^");
        assert_eq!(html, "<em>Italic</em>");
    }
    #[test]
    fn test_link_and_code() {
        let mut converter = Converter::new();
        let html = converter.convert_to_html("\\Code\\");
        assert_eq!(html, "<code>Code</code>");
        let html = converter.convert_to_html("~~Link~~Link~~");
        assert_eq!(html, "<a href='Link'>Link</a>");
        let html = converter.convert_to_html("---");
        assert_eq!(html, "<hr>");
    }
    #[test]
    fn test_list_and_line() {
        let mut converter = Converter::new();
        let html = converter.convert_to_html("> List");
        assert_eq!(html, "<ul><li>List</li></ul>");
        let html = converter.convert_to_html("Line");
        assert_eq!(html, "Line<br>");
    }
}
