use proconio::input;

fn main() {
    input! {
        n: usize,
        cl: [(String, usize); n],
    }

    let mut s = String::new();

    let mut cnt = 0;
    for i in 0..n {
        let (c, l) = &cl[i];
        let l = *l;
        for j in 0..l {
            s.push_str(c);
            cnt += 1;
            if cnt > 100 {
                break;
            }
        }
    }

    if cnt <= 100 {
        println!("{}", s);
    } else {
        println!("Too Long");
    }
}
