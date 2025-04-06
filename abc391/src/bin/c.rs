use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    // initialize
    let mut house = std::collections::BTreeMap::new();
    let mut p2h = std::collections::BTreeMap::new();
    for i in 1..=n as i32 {
        house.insert(i, 1);
        p2h.insert(i, i);
    }

    let mut ans = 0;
    for _ in 0..q {
        input! {
            q_type: usize,
        }

        match q_type {
            1 => {
                input! {
                    p: i32,
                    h: i32,
                }

                // 元々ハトがいた巣
                let old_h = p2h.get(&p).unwrap();
                // もとの巣からマイナス
                let old_count = house.get_mut(old_h).unwrap();
                if *old_count == 2 {
                    ans -= 1;
                }
                *old_count -= 1;

                // ハト -> 巣
                *p2h.get_mut(&p).unwrap() = h;

                // 巣 -> ハトの数
                let count = house.get_mut(&h).unwrap();
                if *count == 1 {
                    ans += 1;
                }
                *count += 1;
            }
            2 => {
                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}
