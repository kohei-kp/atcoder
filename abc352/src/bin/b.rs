use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }
    let s_c = s.chars().collect::<Vec<char>>();
    let t_c = t.chars().collect::<Vec<char>>();

    let mut ans: Vec<i32> = vec![];

    let mut prev_index = 0;
    for i in 0..s_c.len() {
        let c = s_c[i];

        for j in prev_index..t_c.len() {
            if c == t_c[j] {
                ans.push(j as i32 + 1);
                prev_index = j + 1;
                break;
            }
        }
    }

    for c in ans {
        print!("{} ", c);
    }
}
