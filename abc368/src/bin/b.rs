use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }

    let mut cnt = 0;
    loop {
        a.sort_by(|a, b| b.cmp(a));
        if a[1] <= 0 {
            break;
        }
        a[0] -= 1;
        a[1] -= 1;
        cnt += 1;
    }

    println!("{}", cnt);
}
