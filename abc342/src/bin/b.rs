use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [i32; n],
        q: usize,
        ab: [(i32, i32); q]
    }

    for i in 0..q {
        let (a, b) = ab[i];

        let a_i = p.iter().position(|&x| x == a).unwrap();
        let b_i = p.iter().position(|&x| x == b).unwrap();

        if a_i < b_i {
            println!("{}", a);
        } else {
            println!("{}", b);
        }
    }
}
