use proconio::input;

fn main() {
    input! {
        n: usize,
        q: i64
    }

    let mut hole = vec![];
    for i in 0..n {
        hole.push(i);
    }
    let mut cnt = vec![1; n];
    let mut ans = 0;

    for _ in 0..q {
        input! {
            query_type: usize,
        }

        match query_type {
            1 => {
                input! {
                    mut p: usize,
                    mut h: usize,
                }
                p -= 1;
                h -= 1;

                if cnt[hole[p]] >= 2 {
                    ans -= 1;
                }
                cnt[hole[p]] -= 1;
                if cnt[hole[p]] >= 2 {
                    ans += 1;
                }
                hole[p] = h;
                if cnt[hole[p]] >= 2 {
                    ans -= 1;
                }
                cnt[hole[p]] += 1;
                if cnt[hole[p]] >= 2 {
                    ans += 1;
                }
            }
            2 => {
                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}
