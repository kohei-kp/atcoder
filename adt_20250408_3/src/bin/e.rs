use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let mut ans = 0;

    for i in 1.. {
        let iii = i * i * i;
        if iii > n {
            break;
        }
        let s1 = iii.to_string();
        let mut s2 = iii.to_string().chars().collect::<Vec<_>>();
        s2.reverse();
        if s1 == s2.iter().collect::<String>() {
            ans = i * i * i;
        }
    }

    println!("{}", ans);
}
