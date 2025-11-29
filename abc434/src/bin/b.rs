use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut types = vec![0; m];
    let mut counts = vec![0; m];

    for i in 0..n {
        input! {
            ab: (usize, usize)
        }

        let bt = ab.0;
        let w = ab.1;
        types[bt - 1] += w;
        counts[bt - 1] += 1;
    }

    for i in 0..m {
        println!("{:.20}", types[i] as f64 / counts[i] as f64);
    }
}
