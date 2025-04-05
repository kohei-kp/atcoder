use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut sum_l = 0;
    let mut sum_r = 0;
    let mut l = vec![];
    let mut r = vec![];
    for _ in 0..n {
        input! {
            _l: i64,
            _r: i64,
        }
        l.push(_l);
        r.push(_r);
        sum_l += _l;
        sum_r += _r;
    }

    if sum_l > 0 || 0 > sum_r {
        println!("No");
        return;
    }
    println!("Yes");

    let mut ans = l.clone();
    let mut rem: i64 = -sum_l;
    for i in 0..n {
        let can_add = r[i] - l[i];
        if can_add < rem {
            ans[i] = r[i];
            rem -= can_add;
        } else {
            ans[i] += rem;
            break;
        }
    }

    ans.iter().for_each(|x| {
        print!("{} ", x);
    });
}
