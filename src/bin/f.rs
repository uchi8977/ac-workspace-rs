use procon::template::*;

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(solve)
        .unwrap()
        .join()
        .unwrap();
}

fn solve() {
    input! {}
}
