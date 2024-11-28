use proconio::input;

fn main() {
    input! {
        arr: [i32; 4],
    }

    let mut one = 0;
    let mut two = 0;
    let mut three = 0;
    let mut four = 0;

    for a in arr {
        match a {
            1 => one += 1,
            2 => two += 1,
            3 => three += 1,
            4 => four += 1,
            _ => unreachable!(),
        }
    }

    let mut count = 0;

    count += one / 2;
    count += two / 2;
    count += three / 2;
    count += four / 2;

    println!("{}", count);
}
