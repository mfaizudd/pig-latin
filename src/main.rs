use std::io::stdin;

fn main() {
    println!("Enter some words: ");
    let stdin = stdin();
    let mut sentence = String::new();
    if let Err(_) = stdin.read_line(&mut sentence) {
        println!("Invalid input");
        return;
    }
    let vowels = "aiueo";
    let words = sentence.trim().split_whitespace();
    let mut pig_latin: Vec<String> = Vec::new();
    for word in words {
        let chars = word.char_indices();
        let first_letter = if let Some(c) = chars.map(|(_, c)| c).nth(0) {
            c
        } else {
            return;
        };
        let end = first_letter.len_utf8();
        if vowels.contains(first_letter) {
            pig_latin.push(format!("{}-hay", &word));
            continue;
        }
        pig_latin.push(format!("{}{}ay", &word[end..], &first_letter));
    }
    println!("{}", pig_latin.join(" "));
}
