fn main() {
    let my_string = String::from("hello world");
    let literal = "hello world";
    let slice = &my_string[..6];

    let word = first_word(&my_string);
    let word1 = first_word(literal);
    let word2 = first_word(slice);
    
    println!("The fist word is: {word}");
    println!("word1: {word1}");
    println!("word2: {word2}");

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
