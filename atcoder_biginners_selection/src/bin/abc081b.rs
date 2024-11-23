use proconio::input;

fn main() {
    input! {
        n: i32,
        mut a: [i32; n],
    }
    let mut count = 0;
    loop {
        if a.iter().all(|&x| x % 2 == 0) {
            for x in a.iter_mut() {
                *x /= 2;
            }
            count += 1;
        } else {
            break;
        }
    }

    println!("{}", count);
}
