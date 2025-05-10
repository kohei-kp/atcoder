use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }

    let mut ans = 0;

    let mut cp = a.clone();
    for i in 0..n {
        if check(&cp, m) {
            ans += 1;
            cp.pop();
        } else {
            break;
        }
    }

    println!("{}", ans);
}

fn check(a: &Vec<usize>, m: usize) -> bool {
    let mut flg = true;
    // 1以上m以下の要素がすべて含まれているか
    for i in 1..=m {
        if !a.contains(&i) {
            flg = false;
            break;
        }
    }
    flg
}
