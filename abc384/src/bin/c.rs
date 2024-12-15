use proconio::input;

fn main() {
    let n = 5;
    input! {
        a: [i32; n],
    }

    let mut ranking: Vec<(i32, String)> = vec![];

    for s in 1..=31 {
        let mut name = "".to_string();
        let mut score = 0;

        for i in 0..n {
            if s >> i & 1 != 0 {
                score += a[i];
                name.push_str(&format!("{}", (b'A' + i as u8) as char));
            }
        }
        ranking.push((-score, name));
    }

    ranking.sort();

    ranking.iter().for_each(|(_, name)| {
        println!("{}", name);
    });
}
