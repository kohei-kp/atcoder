use proconio::input;

fn main() {
    input! {
        s: String
    }

    let result = s.chars().filter(|c| *c != '.');
    println!("{}", String::from_iter(result));
}
