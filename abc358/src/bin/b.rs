use proconio::input;

fn main() {
    input! {
        n: usize,
        purchase_time: i32,
        t: [i32; n],
    }

    let mut prev = t[0];
    let mut prev_purchase_finish_time = prev + purchase_time;
    println!("{}", prev_purchase_finish_time);

    for i in 1..n {
        let current = t[i];
        let diff = current - prev;
        let purchase_diff = (prev_purchase_finish_time - current).abs();
        let purchase_finish_time = if diff <= purchase_time {
            prev_purchase_finish_time + purchase_time
        } else {
            prev_purchase_finish_time + purchase_diff + purchase_time
        };
        println!("{}", purchase_finish_time);

        prev = current;
        prev_purchase_finish_time = purchase_finish_time;
    }
}
