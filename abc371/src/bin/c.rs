use proconio::input;
use superslice::*;

fn input_graph(n: usize) -> Vec<Vec<bool>> {
    let mut graph = vec![vec![false; n]; n];
    input! {
        m: usize,
    }
    for _ in 0..m {
        input! {
            mut a: usize,
            mut b: usize,
        }
        a -= 1;
        b -= 1;
        graph[a][b] = true;
        graph[b][a] = true;
    }

    graph
}

fn main() {
    input! {
        n: usize,
    }

    let g = input_graph(n);
    let h = input_graph(n);
    let mut a = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            if j > i {
                input! {
                    _a: i32,
                }
                a[i][j] = _a;
                a[j][i] = _a;
            }
        }
    }

    let mut ans = 1_000_000_000;
    let mut p = vec![0; n];
    for i in 0..n {
        p[i] = i;
    }

    loop {
        let mut now = 0;
        for i in 0..n {
            for j in 0..i {
                if h[i][j] != g[p[i]][p[j]] {
                    now += a[i][j];
                }
            }
        }

        ans = ans.min(now);

        if !p.next_permutation() {
            break;
        }
    }

    println!("{}", ans);
}
