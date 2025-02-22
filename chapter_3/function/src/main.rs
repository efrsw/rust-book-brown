fn main() {
    let x = 5;
    another_function(x, 'h');
}

fn another_function(h: i32, unit_lable: char) {
    println!("The time is: {h}{unit_lable}");
}
