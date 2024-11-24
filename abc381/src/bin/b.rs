use proconio::input;

fn main() {
    input! {
        s: String,
    }

    if s.len() % 2 != 0 {
        println!("No");
        return;
    }

    let chars = s.chars().collect::<Vec<char>>();

    for i in 0..chars.len() / 2 {
        let _i = (i + 1) * 2 - 1;
        if chars[_i] != chars[_i - 1] {
            println!("No");
            return;
        }
    }
    let mut counts = vec![];
    for &ch in &chars {
        if let Some(count) = counts.iter_mut().find(|&&mut (ref c, _)| c == &ch) {
            count.1 += 1;
        } else {
            counts.push((ch, 1));
        }
    }

    let is_all_two_chars = counts.iter().all(|&(_, count)| count == 2);

    if is_all_two_chars {
        println!("Yes");
    } else {
        println!("No");
    }
}
