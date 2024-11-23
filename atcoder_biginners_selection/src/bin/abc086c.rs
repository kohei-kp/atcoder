use proconio::input;

fn main() {
    input! {
        n: i32,
        mut t: [(i32, i32, i32); n],
    }

    for (i, &(t_i, x_i, y_i)) in t.iter().enumerate() {
        if i == 0 {
            // (X,Y)がT以上の数なら、そもそも移動できない
            if x_i + y_i > t_i {
                println!("No");
                return;
            }
        } else {
            // 前の座標との絶対値の差がTより大きい場合、移動できない
            let (t_i_1, x_i_1, y_i_1) = t[i - 1];
            let dt = t_i - t_i_1;
            let dx = (x_i - x_i_1).abs();
            let dy = (y_i - y_i_1).abs();
            if dx + dy > dt || (dx + dy) % 2 != dt % 2 {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
