use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize
    }

    let mut board_s = vec![];
    for _ in 0..n {
        input! {
            s: Chars
        }
        board_s.push(s);
    }
    let mut board_t = vec![];
    for _ in 0..m {
        input! {
            t: Chars
        }
        board_t.push(t);
    }

    for a in 0..=n - m {
        for b in 0..=n - m {
            let mut ok = true;

            for i in 0..m {
                for j in 0..m {
                    if board_s[a + i][b + j] != board_t[i][j] {
                        ok = false;
                    }
                }
            }

            if ok {
                println!("{} {}", a + 1, b + 1);
                return;
            }
        }
    }
}
