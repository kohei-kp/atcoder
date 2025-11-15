use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = vec![];
    for i in 0..n {
        ans.push(vec![0; i + 1]);
        for j in 0..=i {
            if j == 0 || i == j {
                ans[i][j] = 1;
            } else {
                ans[i][j] = ans[i - 1][j - 1] + ans[i - 1][j];
            }
        }
    }

    for i in 0..n {
        for j in 0..ans[i].len() {
            print!("{} ", ans[i][j]);
        }
        println!();
    }
}
