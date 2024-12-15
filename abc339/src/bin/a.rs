use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let split_str = s.split(".").collect::<Vec<&str>>();
    println!("{}", split_str[split_str.len() - 1]);
}
