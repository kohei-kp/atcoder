use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [Usize1; q],
    }

    // グリッドの初期状態
    let mut treemap = std::collections::BTreeMap::new();
    for i in 0..n {
        treemap.insert(i, true);
    }

    //println!("{:?}", treemap);

    let mut ans = 0;
    // クエリの処理
    for i in 0..q {
        let target = a[i];

        let left = *treemap.get(&(target.wrapping_sub(1))).unwrap_or(&true);
        let right = *treemap.get(&(target + 1)).unwrap_or(&true);

        // T -> F
        if let Some(&true) = treemap.get(&target) {
            treemap.insert(target, false);

            // 左右の状態を確認
            if left && right {
                // 左右ともにTならば、ansを1増やす
                ans += 1;
            } else if !left && !right {
                // 左右ともにFならば、ansを1減らす
                ans -= 1;
            }

        // F -> T
        } else {
            // trueに変更
            treemap.insert(target, true);
            // 左右の状態を確認
            // 左右がFならば、ansを1増やす
            if !left && !right {
                ans += 1;
            } else if left && right {
                // 左右ともにTならば、ansを1減らす
                ans -= 1;
            } else if !left || !right {
                // 左右のどちらかがFならば、ansは変化しない
            }
        }
        //println!("{:?}", treemap);
        println!("{}", ans);
    }
}
