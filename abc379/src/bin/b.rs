use proconio::input;

fn main() {
    input! {
        _n: i32,
        k: usize,
        s: String,
    }

    let splits = s.split("X").collect::<Vec<&str>>();

    let results = splits
        .iter()
        .map(|&x| {
            let r = x.to_string().chars().count() / k;
            return r;
        })
        .collect::<Vec<usize>>();

    println!("{}", results.iter().sum::<usize>());
}
