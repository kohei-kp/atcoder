use proconio::input;

fn main() {
    input! {
        k: i32,
        s: String,
        t: String
    }

    if s == t {
        println!("Yes");
        return;
    }

    let mut _s = s.chars().collect::<Vec<_>>();
    let mut _t = t.chars().collect::<Vec<_>>();
    let mut slen = _s.len();
    let mut tlen = _t.len();

    if slen == tlen {
        let mut count = 0;
        for i in 0..slen {
            if _s[i] != _t[i] {
                count += 1;
            }
        }
        println!("{}", if count <= 1 { "Yes" } else { "No" });
    } else if slen + 1 == tlen {
        let mut pc = 0;
        let mut sc = 0;
        while pc < slen {
            if _s[pc] == _t[pc] {
                pc += 1;
            } else {
                break;
            }
        }
        while sc < slen {
            if _s[slen - 1 - sc] == _t[tlen - 1 - sc] {
                sc += 1;
            } else {
                break;
            }
        }
        println!("{}", if pc + sc >= slen { "Yes" } else { "No" });
    } else if slen - 1 == tlen {
        std::mem::swap(&mut _s, &mut _t);
        std::mem::swap(&mut slen, &mut tlen);
        let mut pc = 0;
        let mut sc = 0;
        while pc < slen {
            if _s[pc] == _t[pc] {
                pc += 1;
            } else {
                break;
            }
        }
        while sc < slen {
            if _s[slen - 1 - sc] == _t[tlen - 1 - sc] {
                sc += 1;
            } else {
                break;
            }
        }
        println!("{}", if pc + sc >= slen { "Yes" } else { "No" });
    } else {
        println!("No");
    }
}
