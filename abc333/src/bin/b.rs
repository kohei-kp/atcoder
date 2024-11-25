use proconio::input;

fn main() {
    input!(
        s: String,
        t: String,
    );

    // A B C D E
    // 文字列の1文字目を取得
    let s1 = &s[0..1];
    let t1 = &t[0..1];
    let s2 = &s[1..2];
    let t2 = &t[1..2];

    let vec = vec!['A', 'B', 'C', 'D', 'E'];

    // s1からs2の距離を計算する
    let s1_index = vec
        .iter()
        .position(|&x| x == s1.chars().nth(0).unwrap())
        .unwrap();
    let s2_index = vec
        .iter()
        .position(|&x| x == s2.chars().nth(0).unwrap())
        .unwrap();

    // t1からt2の距離を計算する
    let t1_index = vec
        .iter()
        .position(|&x| x == t1.chars().nth(0).unwrap())
        .unwrap();
    let t2_index = vec
        .iter()
        .position(|&x| x == t2.chars().nth(0).unwrap())
        .unwrap();

    let s_distance = if s2_index < s1_index {
        s1_index - s2_index
    } else {
        s2_index - s1_index
    };
    let t_distance = if t2_index < t1_index {
        t1_index - t2_index
    } else {
        t2_index - t1_index
    };
    // 距離が4の場合は1,3は2にする
    let s_distance = match s_distance {
        4 => 1,
        3 => 2,
        _ => s_distance,
    };
    let t_distance = match t_distance {
        4 => 1,
        3 => 2,
        _ => t_distance,
    };

    if s_distance == t_distance {
        println!("Yes");
    } else {
        println!("No");
    }
}
