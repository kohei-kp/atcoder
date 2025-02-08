use proconio::input;

fn main() {
    input! {
        a: [i32; 3],
    }

    if a[0] * a[1] == a[2] {
        println!("Yes");
    } else if a[0] * a[2] == a[1] {
        println!("Yes");
    } else if a[1] * a[2] == a[0] {
        println!("Yes");
    } else {
        println!("No");
    }
}
