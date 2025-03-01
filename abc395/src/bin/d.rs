use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut p2b = vec![];
    let mut b2h = vec![];
    let mut h2b = vec![];
    for i in 0..n {
        p2b.push(i);
        b2h.push(i);
        h2b.push(i);
    }

    for _i in 0..q {
        input! {
            query_type: i32
        }

        match query_type {
            1 => {
                // 鳩を移動させる
                input! {
                    mut a: usize,
                    mut b: usize
                }
                a -= 1;
                b -= 1;
                p2b[a] = h2b[b];
            }
            2 => {
                // 鳩を全部移動させる
                input! {
                    mut a: usize,
                    mut b: usize
                }
                a -= 1;
                b -= 1;
                h2b.swap(a, b);
                b2h[h2b[a]] = a;
                b2h[h2b[b]] = b;
            }
            3 => {
                input! {
                    pn: usize
                }
                // 鳩の位置を出力
                println!("{}", b2h[p2b[pn - 1]] + 1);
            }
            _ => unreachable!(),
        }
    }
}
