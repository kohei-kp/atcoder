use proconio::input;

fn main() {
    input! {
        k: usize,
        g: i32,
        m: i32
    }

    let mut glass = 0;
    let mut mag = 0;

    for i in 0..k {
        if glass == g {
            glass = 0;
        } else if mag == 0 {
            mag = m;
        } else {
            let diff = g - glass;

            if diff > mag {
                glass += mag;
                mag = 0;
            } else {
                glass += diff;
                mag -= diff;
            }
        }
    }

    println!("{} {}", glass, mag);
}
