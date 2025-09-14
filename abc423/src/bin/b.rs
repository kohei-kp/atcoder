use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [usize; n],
    }

    let mut ans1 = 0;
    for i in 0..n {
        if l[i] == 0 {
            ans1 = i + 1;
        } else {
            break;
        }
    }

    let mut ans2 = n;
    for i in (0..n).rev() {
        if l[i] == 0 {
            ans2 = i;
        } else {
            break;
        }
    }

    let cnt1 = ans1 + 1;
    let cnt2 = n - ans2 + 1;
    let ov = if ans1 >= ans2 { ans1 - ans2 + 1 } else { 0 };

    let r = cnt1 + cnt2 - ov;
    let ans = (n + 1) - r;

    println!("{}", ans);
}
