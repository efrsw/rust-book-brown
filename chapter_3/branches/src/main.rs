fn main() {
    let n = false;
    let a = if n {
        5
    } else if !n {
        6
    } else {
        7
    };

    println!("{a}");
}
