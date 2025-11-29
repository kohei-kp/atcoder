use proconio::input;

fn main() {
    input! {
        w: usize,
        b: usize
    }

    let wkg = w * 1000;
    let mut ans = 1;
    loop {
        if ans * b > wkg {
            break;
        }
        ans += 1;
    }
    println!("{}", ans);
}
