use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: String,
    }

    let s = s.chars().collect::<Vec<char>>();
    let mut prev = s[0];
    let mut prev_i = 0;
    let mut v = vec![];
    for i in 1..n {
        if prev != s[i] {
            v.push(&s[prev_i..i]);
            prev_i = i;
            prev = s[i];
        }
    }
    v.push(&s[prev_i..n]);

    let mut all_one_indexes = vec![];
    for i in 0..v.len() {
        let is_all_one = v[i].iter().all(|&c| c == '1');
        if is_all_one {
            all_one_indexes.push(i);
        }
    }

    let k_i = all_one_indexes[k - 1];
    let k_ii = all_one_indexes[k - 2];

    v.insert(k_ii, &v[k_i]);
    v.remove(k_i + 1);

    println!(
        "{}",
        v.iter()
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("")
    );
}
