use proconio::input;

fn main() {
    input! {
        s: i32,
    }

    let num = s.to_string().chars().filter(|&c| c == '1').count();
    println!("{}", num);
}
