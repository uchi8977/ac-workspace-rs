use procon::template::*;

fn main() {
    input! {
        h: usize,
        w: usize,
        x: [[usize; w]; h],
        q: usize,
        rects: [(Usize1, Usize1, usize, usize); q],
    }

    let mut sum = vec![vec![0; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            sum[i + 1][j + 1] = x[i][j] + sum[i + 1][j];
        }
    }
    for i in 0..h {
        for j in 0..w {
            sum[i + 1][j + 1] += sum[i][j + 1];
        }
    }

    for (a, b, c, d) in rects {
        println!("{}", sum[c][d] + sum[a][b] - sum[a][d] - sum[c][b]);
    }
}

// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_h