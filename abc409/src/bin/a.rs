use proconio::input;

fn main() {
    input! {
        n: usize,
        t: String,
        a: String
    }

    let _t = t.chars().collect::<Vec<_>>();
    let _a = a.chars().collect::<Vec<_>>();
    let mut b = false;
    for i in 0..n {
        if _t[i] == 'o' && _a[i] == 'o' {
            b = true;
            break;
        }
    }
    if b {
        println!("Yes");
    } else {
        println!("No");
    }
}
