use proconio::input;

fn main() {
    input! {
        x: usize,
        c: usize,
    }

    let base = 1000;
    let mut ans = 0;
    loop {
        let _ans = ans + base;
        let fee = ans * c / 1000;
        if x < ans + fee {
            ans -= base;
            break;
        } else {
            ans = _ans;
        }
    }
    println!("{}", ans);
}
