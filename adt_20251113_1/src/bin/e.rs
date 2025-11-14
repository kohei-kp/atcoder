use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [[i64; n]; 2]
    }

    // dp[i][j]: 1番目で j= 0 or 1を選べるかどうか
    let mut dp = vec![vec![false; 2]; n];

    dp[0][0] = true;
    dp[0][1] = true;

    for i in 1..n {
        for j in 0..2 {
            let pi = i - 1;
            for pj in 0..2 {
                if !dp[pi][pj] {
                    continue;
                }
                if (a[pj][pi] - a[j][i]).abs() > k {
                    continue;
                }
                dp[i][j] = true;
            }
        }
    }

    if dp[n - 1][0] || dp[n - 1][1] {
        println!("Yes");
    } else {
        println!("No");
    }
}
