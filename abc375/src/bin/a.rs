use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String
    }

    let target = "#.#".to_string();

    let mut ans = 0;
    s.chars().collect::<Vec<char>>().windows(3).for_each(|w| {
        if w.iter().collect::<String>() == target {
            ans += 1;
            return;
        }
    });

    println!("{}", ans);
}
