fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {}
