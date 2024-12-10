use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i32; n],
    }

    let filtered = h[1..]
        .iter()
        .filter_map(|&x| if x > h[0] { Some(x) } else { None })
        .collect::<Vec<i32>>();
    if filtered.is_empty() {
        println!("-1");
    } else {
        println!("{}", h.iter().position(|&x| x == filtered[0]).unwrap() + 1);
    }
}
