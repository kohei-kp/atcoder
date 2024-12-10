use proconio::input;

fn main() {
    input! {
        a: [i32; 9],
        b: [i32; 8]
    }

    let a_sum: i32 = a.iter().sum();
    let b_sum: i32 = b.iter().sum();

    println!("{}", (a_sum - b_sum) + 1);
}
