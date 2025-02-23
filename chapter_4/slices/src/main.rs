fn main() {
    let a = [1, 2, 3, 4];
    let b = &a[0..2];

    let mut s = String::from("Hello world!");
    let word = first_word(&s);

    println!("{word}");
    s.clear();
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    
    &s[..]
}
