use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    let res = [1, 2, 3]
        .iter()
        .filter(|x| **x != a && **x != b)
        .collect::<Vec<_>>();

    if res.len() == 1 {
        println!("{}", res[0]);
    } else {
        println!("-1");
    }
}
