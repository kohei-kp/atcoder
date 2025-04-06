use proconio::input;

fn main() {
    input! {
        mut sx: i64,
        sy: i64,
        mut tx: i64,
        ty: i64,
    }

    if (sx + sy) % 2 == 1 {
        sx -= 1;
    }
    if (tx + ty) % 2 == 1 {
        tx -= 1;
    }

    let x = (tx - sx).abs();
    let y = (ty - sy).abs();

    let mut ans = 0;
    if x < y {
        ans = y;
    } else {
        ans = (x + y) / 2;
    }
    println!("{}", ans);
}
