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
        if vowels.contains(&word[..1]) {
            pig_latin.push(format!("{}-hay", &word));
            continue;
        }
        pig_latin.push(format!("{}{}ay", &word[1..], &word[..1]))
    }
    println!("{}", pig_latin.join(" "));
}
