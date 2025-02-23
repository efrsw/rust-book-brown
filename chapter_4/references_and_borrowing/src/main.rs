fn main() {
    let mut v = vec![1, 2, 3];
    let x = &v[2];
    println!("The value is: {}", *x);
    v.push(4);
}

fn add_bit_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest_string = dst.iter().max_by_key(|s| s.len()).unwrap().len();

    for s in src {
        if s.len() > largest_string {
            dst.push(s.clone());
        }
    }
}
