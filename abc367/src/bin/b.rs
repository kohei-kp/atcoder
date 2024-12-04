use proconio::input;

fn main() {
    input! {
        x: String,
    }

    println!("{}", x.parse::<f32>().unwrap());
}
