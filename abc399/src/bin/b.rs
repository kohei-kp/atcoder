use proconio::input;

fn main() {
    input! {
        n: usize,
        mut p: [i32; n]
    }

    let mut r = 1;

    let mut rank = vec![0; n];
    let mut ranked = vec![false; n];

    while !ranked.iter().all(|&x| x) {
        let mut _p = vec![];
        for i in 0..n {
            if !ranked[i] {
                _p.push(p[i]);
            }
        }
        let max = _p.iter().max().unwrap();
        let mut add = 0;
        for i in 0..p.len() {
            if p[i] == *max {
                rank[i] = r;
                ranked[i] = true;
                add += 1;
            }
        }
        r += add;
    }

    for i in 0..n {
        println!("{}", rank[i]);
    }
}
