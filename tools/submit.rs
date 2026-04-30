use std::{env, fs};
use colored::*;
use arboard::Clipboard;

fn main() {
    let bin_path = env::args().nth(1).expect("usage: submit <path>");
    let template_path = "src/template.rs";

    let template = fs::read_to_string(template_path)
        .expect("failed to read `src/template.rs`");
    let bin = fs::read_to_string(&bin_path)
        .unwrap_or_else(|e| panic!("failed to read `{bin_path}`: {e}"));

    let result = format!(
        "// -------------------- template --------------------\n{template}\n\n// -------------------- main --------------------\n// {bin}"
    );

    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(result).unwrap();

    println!("{} {}", "success:".bright_green(), "code copied to clipboard");
}