use procon::template::*;

fn main() {
    input! {
        n: usize,
        h: [isize; n],
    }

    let mut dp = vec![0; n];
    dp[1] = (h[0] - h[1]).abs();
    for i in 2..n {
        let jump1 = dp[i - 1] + (h[i] - h[i - 1]).abs();
        let jump2 = dp[i - 2] + (h[i] - h[i - 2]).abs();
        dp[i] = jump1.min(jump2);
    }

    println!("{}", dp[n - 1]);
}

// https://atcoder.jp/contests/dp/tasks/dp_a
