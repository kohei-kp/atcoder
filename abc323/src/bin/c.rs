use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; m],
    }

    let mut s = vec![];
    let mut score_list = vec![];
    let mut score_board = vec![];
    for i in 0..n {
        input! {
            ss: Chars
        }
        s.push(ss.clone());
        let mut sum = 0;
        let mut score = vec![];
        for j in 0..m {
            if ss[j] == 'o' {
                sum += a[j];
            }
            score.push((ss[j], a[j]));
        }
        score.sort_by(|a, b| b.1.cmp(&a.1));
        score_board.push(score);
        score_list.push(sum + i as i64 + 1);
    }

    let max_score = score_list.iter().copied().max().unwrap();

    for i in 0..n {
        let mut my_score_sum = score_list[i];
        let my_score = score_board[i].clone();

        if my_score_sum == max_score {
            println!("0");
            continue;
        }

        let mut ans = 0;
        for j in 0..m {
            let score = my_score[j];
            if score.0 == 'x' {
                my_score_sum += score.1;
                ans += 1;
            }
            if my_score_sum > max_score {
                break;
            }
        }
        println!("{}", ans);
    }
}
