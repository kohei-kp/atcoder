use proconio::input;

fn main() {
    input! {
        a: [i32; 7]
    }

    let mut map = std::collections::HashMap::new();

    for i in 0..7 {
        if let Some(v) = map.get_mut(&a[i]) {
            *v += 1;
        } else {
            map.insert(a[i], 1);
        }
    }

    let mut f1 = false;
    let mut f2 = false;
    for (_k, v) in map.iter() {
        if !f1 && *v >= 3 {
            f1 = true;
        } else if !f2 && *v >= 2 {
            f2 = true;
        }
    }

    println!("{}", if f1 && f2 { "Yes" } else { "No" });
}
