use proconio::input;

fn main() {
    input! {
        n: i32,
        mut a: [i32; n],
    }

    a.sort_by(|a, b| b.cmp(a));

    let mut alice = 0;
    let mut bob = 0;

    for (i, &x) in a.iter().enumerate() {
        if i % 2 == 0 {
            alice += x;
        } else {
            bob += x;
        }
    }

    println!("{}", alice - bob);
}
