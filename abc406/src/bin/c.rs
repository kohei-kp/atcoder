use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [i64; n],
    }

    let mut rle = vec![];
    let mut cnt = 0;
    let mut is_increasing = p[0] < p[1];
    for i in 0..n - 1 {
        if p[i] < p[i + 1] {
            if is_increasing {
                cnt += 1;
                continue;
            }
            rle.push(('>', cnt));
            is_increasing = true;
            cnt = 1;
        } else {
            if !is_increasing {
                cnt += 1;
                continue;
            }
            rle.push(('<', cnt));
            is_increasing = false;
            cnt = 1;
        }
    }

    if is_increasing {
        rle.push(('<', cnt));
    } else {
        rle.push(('>', cnt));
    }

    let mut ans: i64 = 0;

    for i in 0..rle.len() {
        if rle[i].0 == '>' {
            let mut l = 0;
            let mut r = 0;
            if 0 < i {
                l = rle[i - 1].1;
            }
            if i + 1 < rle.len() {
                r = rle[i + 1].1;
            }
            ans += l * r;
        }
    }

    println!("{}", ans);
}
