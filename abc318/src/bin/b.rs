use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut grid = vec![vec![false; 100]; 100];
    let mut ans = 0;

    for _ in 0..n {
        input! {
            x1: usize,
            x2: usize,
            y1: usize,
            y2: usize,
        }

        for yp in y1..y2 {
            for xp in x1..x2 {
                if !grid[yp][xp] {
                    grid[yp][xp] = true;
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
