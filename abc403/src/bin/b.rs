use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: Chars,
        u: Chars
    }

    if t.iter().filter(|&&c| c == '?').count() != 4 {
        println!("No");
        return;
    }

    let n = t.len();
    let m = u.len();
    for i in 0..=n - m {
        let mut match_ok = true;
        for j in 0..m {
            if t[i + j] != '?' && t[i + j] != u[j] {
                match_ok = false;
                break;
            }
        }
        if match_ok {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
