use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let without_io = s
        .split("io")
        .filter(|&x| !x.is_empty())
        .collect::<Vec<&str>>();

    let ans: usize = without_io.iter().map(|x| x.len()).sum();
    println!("{}", ans);
}
