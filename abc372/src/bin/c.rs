use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut s: String,
        queries: [(i32, char); q]
    }

    let mut ans = 0;

    for i in 0..n - 2 {
        add(i as i32, 1, &mut s, &mut ans);
    }

    for qi in 0..q {
        let (i, c) = queries[qi];
        add3(i - 1, -1, &mut s, &mut ans);
        s.replace_range((i - 1) as usize..i as usize, &c.to_string());
        add3(i - 1, 1, &mut s, &mut ans);
        println!("{}", ans);
    }
}

fn add(i: i32, co: i32, s: &mut String, ans: &mut i32) {
    if i < 0 || i as usize + 3 > s.len() {
        return;
    }
    if &s[i as usize..(i + 3) as usize] == "ABC" {
        *ans += co;
    }
}

fn add3(i: i32, co: i32, s: &mut String, ans: &mut i32) {
    add(i - 2, co, s, ans);
    add(i - 1, co, s, ans);
    add(i, co, s, ans);
}
