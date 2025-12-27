use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }

    let mut ran: Vec<(i64, i64)> = vec![];

    for x in a {
        if let Some(last) = ran.last_mut() {
            if last.0 == x {
                last.1 += 1;
                // 4つ揃ったら削除する
                if last.1 == 4 {
                    ran.pop();
                }
            } else {
                ran.push((x, 1));
            }
        } else {
            ran.push((x, 1));
        }
    }

    let ans: i64 = ran.iter().map(|&(_, c)| c).sum();
    println!("{}", ans);
}
