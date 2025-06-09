use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        a: [usize; t],
    }

    let mut vertical = vec![n; n];
    let mut horizontal = vec![n; n];
    let mut cross = vec![n; 2];

    let mut count = 0;
    let mut bingo = false;

    for i in 0..t {
        let value = a[i];
        let x = (value - 1) / n;
        let y = (value - 1) % n;
        vertical[x] -= 1;
        horizontal[y] -= 1;
        if x == y {
            cross[0] -= 1;
        }
        if x + y == n - 1 {
            cross[1] -= 1;
        }
        count += 1;
        if vertical[x] == 0 || horizontal[y] == 0 || cross[0] == 0 || cross[1] == 0 {
            bingo = true;
            break;
        }
    }

    if count <= t && bingo {
        println!("{}", count);
    } else {
        println!("-1");
    }
}
