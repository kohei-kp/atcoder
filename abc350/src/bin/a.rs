use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let number = &s[s.len() - 3..s.len()].parse::<i32>().unwrap();
    if number <= &349 && number != &316 && number != &000 {
        println!("Yes");
    } else {
        println!("No");
    }
}
