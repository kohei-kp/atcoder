use proconio::input;

struct Box {
    x: i32,
    y: i32,
    z: i32,
    x2: i32,
    y2: i32,
    z2: i32,
}

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        e: i32,
        f: i32,
        g: i32,
        h: i32,
        i: i32,
        j: i32,
        k: i32,
        l: i32,
    }

    let box1 = Box {
        x: a,
        y: b,
        z: c,
        x2: d,
        y2: e,
        z2: f,
    };

    let box2 = Box {
        x: g,
        y: h,
        z: i,
        x2: j,
        y2: k,
        z2: l,
    };

    // (x,y,z) (x2,y2,z2) => これで直方体

    let x_overlap = if box1.x > box2.x {
        box1.x < box2.x2
    } else {
        box2.x < box1.x2
    };
    let y_overlap = if box1.y > box2.y {
        box1.y < box2.y2
    } else {
        box2.y < box1.y2
    };
    let z_overlap = if box1.z > box2.z {
        box1.z < box2.z2
    } else {
        box2.z < box1.z2
    };
    if x_overlap && y_overlap && z_overlap {
        println!("Yes");
    } else {
        println!("No");
    }
}
