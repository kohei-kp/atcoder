use proconio::input;

fn main() {
    input! {
        mut x: i32,// 高橋
        mut y: i32, // 青木
        z: i32,
    }

    if x < y {
        println!("No");
        return;
    }

    for _ in 0..1000 {
        if y * z == x {
            println!("Yes");
            return;
        }
        x += 1;
        y += 1;
    }

    println!("No");
}
