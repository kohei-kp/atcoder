use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    let chars: Vec<char> = s.chars().collect();

    let mut chunk_num = 1;
    while chunk_num < s.len() {
        let mut chunk = vec![];
        chars.chunks(chunk_num).for_each(|c| chunk.push(c));

        let mut _s: Vec<String> = vec![String::new(); chunk_num];
        chunk.iter().for_each(|c| {
            for i in 0..c.len() {
                _s[i].push(c[i]);
            }
        });

        if _s.contains(&t) {
            println!("Yes");
            return;
        }
        chunk_num += 1;
    }
    println!("No");
}
