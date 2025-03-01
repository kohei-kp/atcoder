use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut grid = vec![vec![""; n]; n];

    for i in 0..n {
        let j = n + 1 - i - 1;
        if i > j {
            continue;
        } else if i <= j {
            if i % 2 == 1 {
                for k in i..j {
                    for l in i..j {
                        grid[k][l] = ".";
                    }
                }
            } else {
                for k in i..j {
                    for l in i..j {
                        grid[k][l] = "#";
                    }
                }
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            print!("{}", grid[i][j]);
        }
        println!();
    }
}
