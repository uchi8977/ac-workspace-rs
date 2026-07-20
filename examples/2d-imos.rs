use procon::template::*;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        rects: [(Usize1, Usize1, usize, usize); n],
    }

    let mut imos = vec![vec![0isize; w + 1]; h + 1];
    for (a, b, c, d) in rects {
        imos[a][b] += 1;
        imos[a][d] -= 1;
        imos[c][b] -= 1;
        imos[c][d] += 1;
    }
    for i in 0..h {
        for j in 1..w {
            imos[i][j] += imos[i][j - 1];
        }
    }
    for i in 1..h {
        for j in 0..w {
            imos[i][j] += imos[i - 1][j];
        }
    }

    for row in &imos[..h] {
        println!("{}", row[..w].iter().join(" "));
    }
}

// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_i
