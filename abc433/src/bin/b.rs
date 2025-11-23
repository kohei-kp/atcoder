use proconio::input;

fn main() {
    input! {
        n: usize,
        a:[i32; n]
    }

    for i in 0..n {
        if i == 0 {
            println!("-1");
            continue;
        }
        let now = a[i];
        let mut near_index = -1;
        for j in 0..i {
            if now < a[j] {
                near_index = j as i32;
            }
        }
        if near_index == -1 {
            println!("-1");
        } else {
            println!("{}", near_index + 1);
        }
    }
}
