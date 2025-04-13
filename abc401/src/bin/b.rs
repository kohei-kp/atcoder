use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut ans = 0;
    let mut logged_in = false;
    for i in 0..n {
        let s = s[i].clone();

        match s.as_str() {
            "login" => {
                logged_in = true;
            }
            "logout" => {
                logged_in = false;
            }
            "public" => {}
            "private" => {
                if !logged_in {
                    ans += 1;
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{}", ans);
}
