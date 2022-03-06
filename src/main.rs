use std::io::stdin;

fn main() {
    let stdin = stdin();
    let mut sentence = String::new();
    if let Err(_) = stdin.read_line(&mut sentence) {
        println!("Invalid input");
        return;
    }
    println!("{}", sentence);
}
