use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut max_h = 0;
    let mut min_h = h - 1;
    let mut max_w = 0;
    let mut min_w = w - 1;

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                max_h = max_h.max(i);
                min_h = min_h.min(i);
                max_w = max_w.max(j);
                min_w = min_w.min(j);
            }
        }
    }

    for i in min_h..=max_h {
        for j in min_w..=max_w {
            if s[i][j] == '.' {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
