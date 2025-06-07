use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort_by(|x, y| y.cmp(x));

    let mut x = 0;
    for (i, &value) in a.iter().enumerate() {
        if value >= i + 1 {
            x = i + 1;
        } else {
            break;
        }
    }

    println!("{}", x);
}
