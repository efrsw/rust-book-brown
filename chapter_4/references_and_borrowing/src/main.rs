fn main() {
    let mut v = vec![1, 2, 3];
    let x = &v[2];
    println!("The value is: {}", *x);
    v.push(4);
}
