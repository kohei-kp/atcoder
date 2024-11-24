use proconio::input;

fn main() {
    input! {
        n: i32,
        s: String,
    }

    if n % 2 == 0 {
        println!("No");
        return;
    }

    if s == "/" {
        println!("Yes");
        return;
    }

    let chars = s.chars().collect::<Vec<char>>();
    let half = ((n + 1) / 2) as usize;
    let one = &chars[0..half - 1];
    let two = &chars[half..n as usize];

    if one.iter().all(|&c| c == '1') && two.iter().all(|&c| c == '2') && chars[half - 1] == '/' {
        println!("Yes");
    } else {
        println!("No");
    }
}
