use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut x = 0;
    while x <= n {
        let mut y = 0;
        while x + y <= n {
            let mut z = 0;
            while x + y + z <= n {
                println!("{} {} {}", x, y, z);
                z += 1;
            }
            y += 1;
        }
        x += 1;
    }
}
