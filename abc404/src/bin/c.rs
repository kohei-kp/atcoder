use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut uf = UnionFind::new(n);
    let mut degree = vec![0; n];
    let mut ans = 0;
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        if uf.equiv(a - 1, b - 1) {
            ans += 1;
        }
        uf.union(a - 1, b - 1);

        degree[a - 1] += 1;
        degree[b - 1] += 1;
    }

    let all_2 = degree.iter().all(|&x| x == 2);
    let connected = (0..n)
        .map(|i| uf.find(i))
        .collect::<std::collections::HashSet<_>>()
        .len()
        == 1;
    if all_2 && connected && ans == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
