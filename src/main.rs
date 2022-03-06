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
    let mut pig_latin = Vec::new();
    for word in words {
        let mut consonants = String::new();
        for c in word.chars() {
            if vowels.contains(c) {
                break;
            }
            consonants.push(c);
        }
        let end = consonants.len();
        if end == 0 {
            pig_latin.push(format!("{}-hay", &word));
            continue;
        }
        pig_latin.push(format!("{}{}ay", &word[end..], &consonants));
    }
    println!("{}", pig_latin.join(" "));
}
