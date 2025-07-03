fn main() {
    let v = vec![1, 2, 3];

    for val in &v {
        let a = val;
        println!("{a}");
    }
}
