use proconio::input;

fn main() {
    input! {
        s: String,
        t: String
    }

    let s = s.chars().collect::<Vec<char>>();
    let t = t.chars().collect::<Vec<char>>();
    let max = s.len().max(t.len());

    let mut ans = 0;

    for i in 0..max {
        let _s = s.get(i).unwrap_or(&' ');
        let _t = t.get(i).unwrap_or(&' ');

        if _s != _t {
            ans += i + 1;
            break;
        }
    }

    println!("{}", ans);
}
