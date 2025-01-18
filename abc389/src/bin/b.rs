use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let mut i = 1;
    loop {
        let ans = fact(i);
        if n == ans {
            println!("{}", i);
            break;
        }
        i += 1;
    }
}

fn fact(n: i64) -> i64 {
    if n == 0 {
        1
    } else {
        fact(n - 1) * n
    }
}
