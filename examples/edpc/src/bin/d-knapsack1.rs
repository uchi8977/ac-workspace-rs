use procon::template::*;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, isize); n],
    }

    let mut dp = vec![vec![isize::MIN; w + 1]; n + 1];
    dp[0][0] = 0;
    for i in 1..=n {
        let (wi, vi) = wv[i - 1];
        for j in 0..=w {
            dp[i][j] = if j < wi {
                dp[i - 1][j]
            } else {
                dp[i - 1][j].max(dp[i - 1][j - wi] + vi)
            };
        }
    }

    println!("{}", dp[n].iter().max().unwrap());
}

// https://atcoder.jp/contests/dp/tasks/dp_d
