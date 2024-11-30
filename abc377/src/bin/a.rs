use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let chars: Vec<char> = s.chars().collect();
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    chars.iter().for_each(|ch| match ch {
        'A' => a += 1,
        'B' => b += 1,
        'C' => c += 1,
        _ => (),
    });

    if a == 1 && b == 1 && c == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
