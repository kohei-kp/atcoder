use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let max = s.iter().max_by_key(|s| s.len()).unwrap();
    let mut result = vec![vec!["*".to_string(); n]; max.len()];

    for i in 0..n {
        let mut j = 0;
        for c in s[i].chars() {
            result[j][n - 1 - i] = c.to_string();
            j += 1;
        }
    }
    for i in 0..max.len() {
        while result[i].last().unwrap() == "*" {
            result[i].pop();
        }
        println!("{}", result[i].join(""));
    }
}
