fn main() {
    let a = Box::new(5);
    let b = add_one(*a);
    println!("{}{}", b, a);
}

fn add_one(mut name: u32) -> u32 {
    name += 1;
    name
}
