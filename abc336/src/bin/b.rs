use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    let mut b = format!("{:b}", n).chars().collect::<Vec<_>>();
    b.reverse();

    let mut count = 0;
    for i in 0..b.len() {
        if b[i] == '0' {
            count += 1;
        } else {
            break;
        }
    }

    println!("{}", count);
}
