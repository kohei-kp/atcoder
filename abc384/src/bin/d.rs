use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: i64,
        a: [i64; n],
    }

    let mut sums: Vec<i64> = vec![];

    for i in 0..n {
        for j in i..n {
            sums.push(a[i..=j].iter().sum());
        }
    }
    sums.sort_by(|a, b| b.cmp(a));

    let mut i = 0;
    while i < sums.len() && s > 0 {
        loop {
            if s - sums[i] < 0 {
                i += 1;
                break;
            }
            s -= sums[i];
            println!("{}", s);
        }
    }

    println!("{}", if s == 0 { "Yes" } else { "No" });
}
