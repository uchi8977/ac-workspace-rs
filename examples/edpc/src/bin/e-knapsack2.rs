use procon::template::*;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }

    let v = 100_000;
    let mut dp = vec![vec![w + 1; v + 1]; n + 1];
    dp[0][0] = 0;
    for i in 1..=n {
        let (wi, vi) = wv[i - 1];
        for j in 0..=v {
            dp[i][j] = if j < vi {
                dp[i - 1][j]
            } else {
                dp[i - 1][j].min(dp[i - 1][j - vi] + wi)
            };
        }
    }

    println!("{}", dp[n].iter().rposition(|&wi| wi <= w).unwrap());
}

// https://atcoder.jp/contests/dp/tasks/dp_e
