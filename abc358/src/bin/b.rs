use proconio::input;

fn main() {
    input! {
        n: usize,
        purchase_time: i32,
        t: [i32; n],
    }

    let mut x = 0;
    for i in 0..n {
        if t[i] < x {
            x = x + purchase_time;
        } else {
            x = t[i] + purchase_time;
        }
        println!("{}", x);
    }
}
