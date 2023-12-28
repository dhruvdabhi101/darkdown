pub struct Converter {
    in_code_block: bool,
}

impl Converter {
    pub fn new() -> Self {
        Converter {
            in_code_block: false,
        }
    }

    pub fn convert_to_html(&mut self, markup: &str) -> String {
        let mut html = String::new();

        for line in markup.lines() {
            if line.starts_with("@") {
                html.push_str(&format!("<h1>{}</h1>", &line[2..]));
            } else if line.starts_with("**") && line.ends_with("**") {
                html.push_str(&format!("<strong>{}</strong>", &line[2..line.len() - 2]));
            } else if line.starts_with("*") && line.ends_with("*") {
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
            } else {
                html.push_str(&format!("{}<br>", line));
            }
        }

        if self.in_code_block {
            html.push_str("</pre>");
        }

        html
    }
}
