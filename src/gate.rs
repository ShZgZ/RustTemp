pub fn and_gate(x1: i32, x2: i32) -> i32 {
    let w = vec!(0.5, 0.6);
    let bias = -0.7;
    let result = fire(&times(&w, &vec!(x1 as f32, x2 as f32)).unwrap(), bias);

    if result <= 0.0 {
        0
    } else {
        1
    }
}

pub fn or_gate(x1: i32, x2: i32) -> i32 {
    let w = vec!(0.2, 0.2);
    let bias = -0.1;
    let result = fire(&times(&w, &vec!(x1 as f32, x2 as f32)).unwrap(), bias);

    if result <= 0.0 {
        0
    } else {
        1
    }
}

pub fn not_and_gate (x1: i32, x2: i32) -> i32 {
    let w = vec!(-0.5, -0.5);
    let bias = 0.7;
    let result = fire(&times(&w, &vec!(x1 as f32, x2 as f32)).unwrap(), bias);

    if result <= 0.0 {
        0
    } else {
        1
    }
}

fn times(x: &Vec<f32>, y: &Vec<f32>) -> Option<Vec<f32>> {

    if x.len() != y.len() {
        None
    } else {
        let mut vec = Vec::new();
        for i in 0..x.len() {
            vec.push(x.get(i).unwrap() * y.get(i).unwrap());
        }
        Some(vec)
    }
}

fn fire(res: &Vec<f32>, bias: f32) -> f32 {
    res.iter().fold(0.0, |sum, x| sum + x) + bias
}
