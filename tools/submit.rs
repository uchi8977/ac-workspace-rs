use base64::{Engine as _, engine::general_purpose::STANDARD};
use colored::*;
use std::{
    env, fs,
    io::{self, Write},
};

fn copy(text: &str) -> io::Result<()> {
    let encoded = STANDARD.encode(text);
    print!("\x1b]52;c;{encoded}\x07");
    io::stdout().flush()
}

fn main() {
    let bin_path = env::args().nth(1).expect("usage: submit <path>");
    let template_path = "src/template.rs";

    let template =
        fs::read_to_string(template_path).expect("failed to read `src/template.rs`");
    let bin = fs::read_to_string(&bin_path)
        .unwrap_or_else(|e| panic!("failed to read `{bin_path}`: {e}"));

    let result = format!(
        "// -------------------- template --------------------\n{template}\n\n// -------------------- main --------------------\n// {bin}"
    );

    copy(&result).expect("failed to request clipboard copy");

    println!(
        "{} {}",
        "success:".bright_green(),
        "clipboard copy requested"
    );
}
