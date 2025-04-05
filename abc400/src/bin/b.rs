use proconio::input;

fn main() {
    input! {
        n: i64,
        m: i64
    }

    let mut sum = 0;
    for i in 0..=m {
        if let Some(res) = n.checked_pow(i as u32) {
            sum += res;
        } else {
            println!("inf");
            return;
        }
    }
    if sum > 1e9 as i64 {
        println!("inf");
        return;
    }
    println!("{}", sum);
}
