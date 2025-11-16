use proconio::input;

fn main() {
    input! {
        mut x: i64
    }

    // 入力と、カウントしているNを昇順ソートして、初めに一致したものが最小
    // Xのソートしたものが00123など前0があってもNにそれがないので省ける。
    let mut n = 0;
    loop {
        if f(n) == f(x) {
            println!("{}", n);
            return;
        }
        n += 1;
    }
}

fn f(x: i64) -> String {
    let s = x.to_string();
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();

    chars.into_iter().collect()
}
