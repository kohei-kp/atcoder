use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        a: [i32; n],
    }

    let mut ans = vec![];
    for i in 0..n {
        if a[i] % k == 0 {
            ans.push(a[i] / k);
        }
    }
    ans.sort();
    for i in 0..ans.len() {
        print!("{} ", ans[i]);
    }
}
