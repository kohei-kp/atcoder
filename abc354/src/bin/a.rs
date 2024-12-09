use proconio::input;

fn main() {
    input! {
        h: i32
    }

    let mut count = 0;
    let mut plant_h = 0;

    while plant_h <= h {
        plant_h += 2i32.pow(count);
        count += 1;
    }
    println!("{}", count);
}
