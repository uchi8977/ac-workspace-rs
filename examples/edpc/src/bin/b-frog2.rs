use procon::template::*;

fn main() {
    input! {
        n: usize,
        k: usize,
        h: [isize; n],
    }

    let mut dp = vec![isize::MAX; n];
    dp[0] = 0;
    for i in 1..n {
        for j in i.saturating_sub(k)..i {
            let cost = dp[j] + (h[i] - h[j]).abs();
            dp[i].chmin(cost);
        }
    }

    println!("{}", dp[n - 1]);
}

// https://atcoder.jp/contests/dp/tasks/dp_b
