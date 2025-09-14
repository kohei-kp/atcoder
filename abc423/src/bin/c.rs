use proconio::input;

fn main() {
    input! {
        n: usize,
        r: usize,
        l: [usize; n],
    }

    let left_lock = l[0..r].to_vec();
    let mut flg = false;
    let mut cnt = 0;

    let mut is_all_one = true;

    for i in 0..r {
        if flg == false && left_lock[i] == 0 {
            flg = true;
        }
        if left_lock[i] == 0 {
            is_all_one = false;
        }

        // はじめて0が出たらカウント開始
        if flg {
            if left_lock[i] == 0 {
                cnt += 1;
            } else {
                cnt += 2;
            }
        }
    }
    if flg == false && !is_all_one {
        cnt += left_lock.len();
    }

    // Rより右側のロック回数
    // 逆順にして左側と同じ処理をする
    let right_lock = l[r..n].to_vec().into_iter().rev().collect::<Vec<usize>>();
    flg = false;
    is_all_one = true;
    for i in 0..n - r {
        if flg == false && right_lock[i] == 0 {
            flg = true;
        }
        if right_lock[i] == 0 {
            is_all_one = false;
        }

        // はじめて0が出たらカウント開始
        if flg {
            if right_lock[i] == 0 {
                cnt += 1;
            } else {
                cnt += 2;
            }
        }
    }

    if flg == false && !is_all_one {
        cnt += right_lock.len();
    }

    println!("{}", cnt);
}
