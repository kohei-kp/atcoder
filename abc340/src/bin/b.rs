use proconio::input;

fn main() {
    input! {
        q: usize,
        qs: [(i32, i32); q],
    }
    let mut ans: Vec<i32> = vec![];

    for (a, s) in qs {
        if a == 1 {
            ans.push(s);
        } else {
            println!("{}", ans[ans.len() - s as usize]);
        }
    }
}
