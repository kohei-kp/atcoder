use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut ran: Vec<(i64, i64)> = vec![];
    let mut count = 1;
    let mut prev = s[0];
    for i in 1..s.len() {
        let c = s[i];
        if c == prev {
            count += 1;
        } else if c != prev {
            let num: i64 = ([prev].iter().collect::<String>()).parse().unwrap();
            ran.push((num, count));
            count = 1;
        }
        prev = c;
    }
    let num: i64 = ([prev].iter().collect::<String>()).parse().unwrap();
    ran.push((num, count));

    if ran.len() == 1 {
        println!("0");
        return;
    }

    let mut ans = 0;
    let mut prev = ran[0];
    for i in 1..ran.len() {
        // +1になるかどうか
        if prev.0 + 1 != ran[i].0 {
            prev = ran[i];
            continue;
        }

        ans += std::cmp::min(prev.1, ran[i].1);
        prev = ran[i];
    }

    println!("{}", ans);
}
