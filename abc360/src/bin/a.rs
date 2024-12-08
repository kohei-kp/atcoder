use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let chars: Vec<char> = s.chars().collect();

    let r_index = chars.iter().position(|&c| c == 'R').unwrap();
    let m_index = chars.iter().position(|&c| c == 'M').unwrap();

    println!("{}", if r_index < m_index { "Yes" } else { "No" });
}
