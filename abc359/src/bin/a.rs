use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    println!("{}", s.iter().filter(|&c| c == "Takahashi").count());
}
