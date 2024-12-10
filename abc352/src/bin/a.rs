use proconio::input;

fn main() {
    input! {
           _: usize,
           x: i32,
           y: i32,
           z: i32,
    }

    let mut a = (x.min(y)..x.max(y)).collect::<Vec<i32>>();
    if x > y {
        a.reverse();
    }

    for i in 0..a.len() {
        if a[i] == z {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
