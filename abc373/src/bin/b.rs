use proconio::input;

fn main() {
    input! {
        s: String
    }

    let mut current_index = s.find("A").unwrap();
    let chars = s.chars().collect::<Vec<char>>();
    let search_order = ('A'..='Z').collect::<Vec<char>>();
    let mut search_index = 1;
    let mut move_distance = 0;
    loop {
        let current = chars[current_index];
        if current == 'Z' {
            break;
        }

        let target = search_order[search_index];
        let target_index = chars.iter().position(|&c| c == target).unwrap();
        move_distance += (target_index as i32 - current_index as i32).abs();
        current_index = chars.iter().position(|&c| c == target).unwrap();
        search_index += 1;
    }

    println!("{}", move_distance);
}
