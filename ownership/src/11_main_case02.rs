fn main() {
    // 传入一个字符串，找出第一个空格前的单词并返回

    let s = String::from("Hello World");
    let word_index = first_word(&s);

    println!("{}", word_index);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}