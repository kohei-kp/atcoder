use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [f64; n]
    }

    println!(
        "{}",
        if is_geometric_sequence(&a) {
            "Yes"
        } else {
            "No"
        }
    );
}

fn is_geometric_sequence(a: &[f64]) -> bool {
    let ratio = a[1] / a[0];
    for i in 1..a.len() {
        if a[i] != a[i - 1] * ratio {
            return false;
        }
    }
    true
}
