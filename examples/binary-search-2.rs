use procon::template::*;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    println!("{}", a.partition_point(|&v| v < x) + 1);
}

// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_k