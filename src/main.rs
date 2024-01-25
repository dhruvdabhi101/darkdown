use crate::converter::converter::Converter;
use clap::Parser;
use std::fs::{self, File};
pub mod converter;

#[derive(Default, Parser, Debug)]
/// Convert a .dd file to .html
struct Config {
    /// Input file name to convert
    input_file: String,
}

fn main() {
    let args = Config::parse();
    let file_name = &args.input_file;
    let file_content = std::fs::read_to_string(file_name).unwrap();
    let file_name_without_ext = file_name.split(".").collect::<Vec<&str>>()[0];
    let ext = file_name.split(".").collect::<Vec<&str>>()[1];
    if ext != "dd" {
        panic!("File extension is not supported");
    } else {
        let html_file_name = format!("{}.html", file_name_without_ext);

        let mut converter = Converter::new();
        let html_output = converter.convert_to_html(&file_content);
        File::create(html_file_name.clone()).expect("creation failed");
        fs::write(html_file_name, html_output).expect("Unable to write file");
    }
}
