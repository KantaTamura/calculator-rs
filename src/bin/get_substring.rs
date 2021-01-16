fn main() {
    let s = "1234567890";

    println!("String:\n{:?}\n", s);
    println!("Chars:\n{:?}\n", s.chars());
    println!("Enumerate:\n{:?}\n", s.chars().enumerate());

    let sub_s = s
        .chars()
        .enumerate()
        .filter(|&(i, _)| i >= 2 && i <=4)
        .fold("".to_string(), |s, (_, c)| format!("{}{}", s, c));

    println!("substring:\n{:?}\n", sub_s);

    let text = "Hello, World!";

    let begin = text.char_indices().nth(0).unwrap().0;
    let end   = text.char_indices().nth(5).unwrap().0;
    let sub_text = &text[begin..end];

    println!("{:?}", sub_text);
}