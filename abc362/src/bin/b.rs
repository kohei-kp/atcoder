use proconio::input;

fn main() {
    input! {
        a: (i32, i32),
        b: (i32, i32),
        c: (i32, i32),
    }

    let dist_a = distance(a, b);
    let dist_b = distance(b, c);
    let dist_c = distance(c, a);

    if dist_a == dist_b + dist_c || dist_b == dist_c + dist_a || dist_c == dist_a + dist_b {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    dx * dx + dy * dy
}
