use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        range: [(Usize1, usize); m],
    }

    let mut walls = vec![0; n + 1];
    for (l, r) in range {
        walls[l] += 1;
        walls[r] -= 1;
    }
    for i in 1..=n {
        walls[i] += walls[i - 1];
    }

    let mut ans = 1_000_000_000;
    for i in 0..n {
        ans = ans.min(walls[i]);
    }
    println!("{}", ans);
}
