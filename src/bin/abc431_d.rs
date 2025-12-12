use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        whb:[(usize,usize,usize);n]
    }
    let sum_w: usize = whb.iter().map(|(w, _, _)| w).sum();

    const MAX_WEIGHT: usize = 500 * 500;
    const INF: i64 = 2_000_000_000_000_000_000;
    let mut dp = vec![-INF; MAX_WEIGHT + 1];
    dp[0] = 0;
    for &(w, h, b) in &whb {
        let mut ndp = dp.clone();
        for j in 0..=MAX_WEIGHT {
            if j + w <= MAX_WEIGHT {
                ndp[j + w] = max(ndp[j + w], dp[j] + h as i64);
            }
            ndp[j] = max(ndp[j], dp[j] + b as i64);
        }
        dp = ndp;
    }

    let mut ans = -INF;
    for (j, &dp_j) in dp.iter().enumerate().take(MAX_WEIGHT.min(sum_w) + 1) {
        let body_weight = sum_w - j;
        if body_weight >= j {
            ans = max(ans, dp_j);
        }
    }
    println!("{}", ans);
}
