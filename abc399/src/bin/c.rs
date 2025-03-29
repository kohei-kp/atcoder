use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut uf = UnionFind::new(n);
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
    }

    println!("{}", ans);
}
