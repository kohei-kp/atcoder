use proconio::input;

fn main() {
    input! {
        n: usize,
        tv: [(i32, i32); n],
    }

    let mut prev = tv[0];
    let mut water = prev.1;
    for i in 1..n {
        let (t, v) = tv[i];
        water -= t - prev.0;
        if water < 0 {
            water = 0;
        }
        water += v;
        prev = tv[i];
    }

    println!("{}", water);
}
