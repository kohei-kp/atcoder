use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        grid: [Chars; n]
    }

    let mut ans = vec![];

    for si in 0..n - m + 1 {
        for sj in 0..n - m + 1 {
            let mut sub = vec![vec!['.'; m]; m];
            for i in 0..m {
                for j in 0..m {
                    sub[i][j] = grid[si + i][sj + j];
                }
            }
            ans.push(sub.iter().flatten().join(""));
        }
    }

    ans.sort();
    ans.dedup();
    println!("{}", ans.len());
}
