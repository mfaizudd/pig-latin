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
        let end = match chars.map(|(i,_)| i).nth(0) {
            Some(i) => i,
            None => {
                println!("Please enter some word");
                return;
            },
        };
        if vowels.contains(&word[..end]) {
            pig_latin.push(format!("{}-hay", &word));
            continue;
        }
        pig_latin.push(format!("{}{}ay", &word[end..], &word[..end]));
    }
    println!("{}", pig_latin.join(" "));
}
