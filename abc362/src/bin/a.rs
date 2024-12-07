use proconio::input;

fn main() {
    input! {
        r: i32,
        g: i32,
        b: i32,
        c: String
    }

    let result = match c.as_str() {
        "Red" => g.min(b),
        "Green" => r.min(b),
        "Blue" => r.min(g),
        _ => unreachable!(),
    };

    println!("{}", result);
}
