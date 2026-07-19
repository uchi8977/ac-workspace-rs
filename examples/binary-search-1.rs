use procon::template::*;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    let mut ng = -1;
    let mut ok = n as isize;

    while ok - ng > 1 {
        let mid = (ok + ng) / 2;
        if a[mid as usize] >= x {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok + 1);
}

// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_k