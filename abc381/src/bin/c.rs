use proconio::input;

fn main() {
    input! {
        n: i32,
        s: String,
    }

    let slash_indexes = s
        .char_indices()
        .filter(|(_, c)| *c == '/')
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    if slash_indexes.len() == 0 {
        println!("0");
        return;
    }

    let chars = s.chars().collect::<Vec<char>>();
    let mut results = vec![];

    for i in 0..slash_indexes.len() {
        let si = slash_indexes[i];
        results.push(1);

        if si == 0 || si == n as usize - 1 {
            continue;
        }

        let mut left = si - 1;
        let mut right = si + 1;
        loop {
            if chars[left] != '1' {
                break;
            }
            if chars[right] != '2' {
                break;
            }
            results[i] += 2;

            if left == 0 || right == n as usize - 1 {
                break;
            }

            left -= 1;
            right += 1;
        }
    }

    println!("{}", results.iter().max().unwrap());
}
