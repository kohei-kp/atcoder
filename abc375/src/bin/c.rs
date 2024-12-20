use std::{cmp::min, mem::swap};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }

    let mut b = a.clone();

    for i in 0..n {
        for j in 0..n {
            let mut ni = i;
            let mut nj = j;
            let k = min(min(ni + 1, nj + 1), min(n - ni, n - nj)) % 4;

            for _ in 0..k {
                swap(&mut ni, &mut nj);
                nj = n - 1 - nj;
            }
            b[ni][nj] = a[i][j];
        }
    }

    for i in 0..n {
        println!("{}", b[i].iter().collect::<String>());
    }
}
