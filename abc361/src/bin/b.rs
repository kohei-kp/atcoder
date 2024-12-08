use proconio::input;

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn get_point() -> Point {
    input! {
        x: i32,
        y: i32,
        z: i32,
    }
    Point { x, y, z }
}

fn solve() -> bool {
    let p1 = get_point();
    let p2 = get_point();
    let p3 = get_point();
    let p4 = get_point();

    if p2.x <= p3.x {
        return false;
    }
    if p4.x <= p1.x {
        return false;
    }
    if p2.y <= p3.y {
        return false;
    }
    if p4.y <= p1.y {
        return false;
    }
    if p2.z <= p3.z {
        return false;
    }
    if p4.z <= p1.z {
        return false;
    }
    true
}

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}
