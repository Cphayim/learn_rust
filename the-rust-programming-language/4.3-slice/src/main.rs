fn main() {
    let s = String::from("hello world");
    let first = first_word(&s);
    println!("{first}");

    let s = "HELLO_WORLD";
    let first = first_word(s);
    println!("{first}");

    let first = first_word(&s);
    println!("{first}");

    let first = first_word(&s[..5]);
    println!("{first}");

    let first = first_word(&s[6..]);
    println!("{first}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}
