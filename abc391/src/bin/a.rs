use proconio::input;

fn main() {
    input! {
        d: String
    }

    match d.as_str() {
        "N" => println!("S"),
        "S" => println!("N"),
        "E" => println!("W"),
        "W" => println!("E"),
        "NE" => println!("SW"),
        "NW" => println!("SE"),
        "SE" => println!("NW"),
        "SW" => println!("NE"),
        _ => unreachable!(),
    }
}
