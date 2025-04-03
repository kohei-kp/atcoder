use proconio::input;

#[derive(Debug, Clone)]
struct Solve {
    n: usize,
    k: usize,
    r: Vec<i32>,
}

fn main() {
    input! {
        n: usize,
        k: usize,
        r: [i32; n],
    }

    solve(Vec::new(), Solve { n, k, r });
}

fn solve(ary: Vec<i32>, params: Solve) {
    // ベースケース
    // ary == nなら
    if ary.len() == params.n {
        // 総和がkの倍数なら出力する
        if ary.iter().sum::<i32>() % params.k as i32 == 0 {
            for i in 0..params.n {
                print!("{} ", ary[i]);
            }
            println!();
        }
        return;
    }

    for i in 1..=params.r[ary.len()] {
        let mut new_ary = ary.clone();
        new_ary.push(i);
        solve(new_ary, params.clone());
    }
}
