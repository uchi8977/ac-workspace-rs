use std::{env, fs};
use colored::*;
use arboard::Clipboard;

fn main() {
    let args: Vec<String> = env::args().collect();
    let bin_path = &args[1];
    let template_path = "src/template.rs";

    let template = fs::read_to_string(template_path).unwrap();
    let bin = fs::read_to_string(bin_path).unwrap();

    let result = format!(
        "// -------------------- template --------------------\n{template}\n\n// -------------------- main --------------------\n// {bin}"
    );

    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(result).unwrap();

    println!("{} {}", "success:".bright_green(), "code copied to clipboard");
}