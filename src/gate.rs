pub fn and_gate(x1: i32, x2: i32) -> i32 {
    let w1 = 0.5;
    let w2 = 0.5;
    let theta = 0.7;

    if w1 * (x1 as f32) + w2 * (x2 as f32) > theta {
        1
    } else {
        0
    }
}

pub fn or_gate(x1: i32, x2: i32) -> i32 {
    let (w1, w2, theta) = (0.2, 0.2, 0.1);

    if x1 as f32 * w1 + x2 as f32 * w2 > theta {
        1
    } else {
        0
    }
}

pub fn not_and_gate (x1: i32, x2: i32) -> i32 {
    let w1 = -0.5;
    let w2 = -0.5;
    let theta = -0.7;

    if w1 * (x1 as f32) + w2 * (x2 as f32) > theta {
        1
    } else {
        0
    }
}
