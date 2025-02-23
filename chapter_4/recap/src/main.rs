fn main() {
    let mut vec = vec![1, 2, 3, 4];
    vec = round_inplace(vec);
    println!("{}", vec[0]);
}

fn round_inplace(mut v: Vec<i32>) -> Vec<i32> {
    for i in 0..v.len() {
        v[i] += 1;
    }

    return v;
}

fn extract(b: &Box<i32>) -> i32 {
    let mut b2: Box<i32> = *b; // Move happens, because Box<i32> doesn't implement Copy trait
    *b2
}
