use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [i64; n],
        b: [i64; m],
    }

    for i in 0..m {
        if a.contains(&b[i]) {
            // b[i]をAから削除
            let index = a.iter().position(|&x| x == b[i]).unwrap();
            a.remove(index);
        }
    }

    for i in 0..a.len() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", a[i]);
    }
}
