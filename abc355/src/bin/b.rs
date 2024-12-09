use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i32; n],
        b: [i32; m],
    }

    let mut c = [a.clone(), b.clone()].concat();
    c.sort();

    let mut count = 0;
    let mut counted = false;
    for i in 0..c.len() {
        for a_i in 0..n {
            if c[i] == a[a_i] {
                count += 1;
                counted = true;
                break;
            }
            if counted && a_i == n - 1 {
                count = 0;
                counted = false;
            }
        }
        if count == 2 {
            break;
        }
    }
    println!("{}", if count == 2 { "Yes" } else { "No" });
}
