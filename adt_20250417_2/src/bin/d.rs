use proconio::input;

struct Point {
    x: i64,
    y: i64,
}
fn main() {
    input! {
        p1: (i64, i64),
        p2: (i64, i64),
        p3: (i64, i64),
    }

    let p1 = Point { x: p1.0, y: p1.1 };
    let p2 = Point { x: p2.0, y: p2.1 };
    let p3 = Point { x: p3.0, y: p3.1 };

    let ab2 = (p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2);
    let bc2 = (p2.x - p3.x).pow(2) + (p2.y - p3.y).pow(2);
    let ca2 = (p3.x - p1.x).pow(2) + (p3.y - p1.y).pow(2);

    if ab2 + bc2 == ca2 || bc2 + ca2 == ab2 || ca2 + ab2 == bc2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
