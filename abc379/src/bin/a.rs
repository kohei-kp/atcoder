use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    let c = n.to_string().chars().collect::<Vec<char>>();

    if c.len() != 3 {
        return;
    }

    let [a, b, c] = [c[0], c[1], c[2]];

    println!("{}{}{} {}{}{}", b, c, a, c, a, b);
}
