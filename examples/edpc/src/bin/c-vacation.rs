use procon::template::*;

fn main() {
    input! {
        n: usize,
        abc: [(usize, usize, usize); n],
    }

    let mut dp = vec![[0usize; 3]; n + 1];
    for (i, (a, b, c)) in abc.into_iter().enumerate() {
        dp[i + 1][0] = dp[i][1].max(dp[i][2]) + a;
        dp[i + 1][1] = dp[i][2].max(dp[i][0]) + b;
        dp[i + 1][2] = dp[i][0].max(dp[i][1]) + c;
    }

    println!("{}", dp[n].iter().max().unwrap());

    /* rolling dp
    let mut dp = [0; 3];
    for (a, b, c) in abc {
        dp = [
            dp[1].max(dp[2]) + a,
            dp[2].max(dp[0]) + b,
            dp[0].max(dp[1]) + c,
        ];
    }

    println!("{}", dp.iter().max().unwrap());
    */
}

// https://atcoder.jp/contests/dp/tasks/dp_c
