use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let keep: HashSet<String> = env::args().skip(1).collect();

    let base = fs::read_to_string("src/base.rs")
        .expect("failed to read `src/base.rs`");

    for entry in fs::read_dir("src/bin").expect("failed to read `src/bin`") {
        let path = entry.expect("failed to read entry").path();
        if path.extension().is_some_and(|s| s == "rs") {
            let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            if keep.contains(stem) {
                println!("skipped: {}", path.display());
                continue;
            }
            fs::write(&path, &base)
                .unwrap_or_else(|e| panic!("failed to write `{}`: {e}", path.display()));
            println!("initialized: {}", path.display());
        }
    }
}