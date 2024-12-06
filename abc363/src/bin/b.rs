use proconio::input;

fn main() {
    input! {
        n: usize,
        t: i32,
        p: usize,
        mut l: [i32; n]
    }

    let mut ans = 0;
    while l.iter().filter(|&x| *x >= t).count() < p {
        l.iter_mut().for_each(|x| *x += 1);
        ans += 1;
    }
    println!("{}", ans);
}
