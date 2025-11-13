use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        a: [usize; k],
        l: [usize; q]
    }

    let mut cell = vec!["".to_string(); n];
    for i in 0..k {
        cell[a[i] - 1] = "#".to_string();
    }

    for i in 0..q {
        let num = l[i];
        let mut cnt = 0;
        for j in 0..cell.len() {
            if cell[j] == "#".to_string() {
                cnt += 1;
            }
            if cnt == num {
                if j + 1 < n {
                    if cell[j + 1] == "".to_string() {
                        cell[j + 1] = "#".to_string();
                        cell[j] = "".to_string();
                    }
                }
            }
        }
    }

    let mut cnt = 1;
    for i in 0..cell.len() {
        if cell[i] == "#".to_string() {
            print!("{} ", cnt);
        }
        cnt += 1;
    }
    println!("");
}
