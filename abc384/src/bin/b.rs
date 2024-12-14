use proconio::input;

fn main() {
    input! {
        n: usize,
        r: i32,
        a: [(i32, i32); n],
    }

    let mut ans = r;
    a.iter().for_each(|(div, rate)| {
        if is_rate_target_with_div1(ans) && *div == 1 {
            ans += rate;
        } else if is_rate_target_with_div2(ans) && *div == 2 {
            ans += rate;
        }
    });
    println!("{}", ans);
}

fn is_rate_target_with_div1(rate: i32) -> bool {
    rate >= 1600 && rate <= 2799
}

fn is_rate_target_with_div2(rate: i32) -> bool {
    rate >= 1200 && rate <= 2399
}
