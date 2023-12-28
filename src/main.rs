use std::{fs::{File, self}, io::Write};

use crate::converter::converter::Converter;

mod converter;

fn main() {
    let custom_markup = r#"
@ Heading 1

**bold**

*Italic*

\Code\


\\\\
Code
\\\\

~~Text~~Link~~
---

This is syntax of my own markup language.
"#;
    let args:Vec<_> = std::env::args().collect();
    if args.len() > 1 {
        let file_name = &args[1];
        let file_content = std::fs::read_to_string(file_name).unwrap();
        let file_name_without_ext = file_name.split(".").collect::<Vec<&str>>()[0];
        let html_file_name = format!("{}.html", file_name_without_ext);


        let mut converter = Converter::new();
        let html_output = converter.convert_to_html(&file_content);
        let mut data_file = File::create(html_file_name.clone()).expect("creation failed");
        fs::write(html_file_name, html_output).expect("Unable to write file");
    }
}
