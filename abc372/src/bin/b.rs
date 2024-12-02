// 解説放送見ながら実装
use proconio::input;

fn main() {
    input! {
        mut m: i64
    }

    let mut ans = vec![];
    let mut i = 10;

    while i >= 0 {
        let mut x = 1;

        for _j in 0..i {
            x *= 3;
        }
        while m >= x {
            m -= x;
            ans.push(i);
        }

        i -= 1;
    }

    println!("{}", ans.len());
    for i in ans {
        print!("{} ", i);
    }
}
