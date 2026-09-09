use std::io;

fn extract_word(string: &String, target_word: usize) -> &str {
    string.split_whitespace().nth(target_word - 1).unwrap_or("")
}

fn main() {
    println!("Input the sentence, that you want to extract Nth word of:");

    let mut raw_input = String::new();
    io::stdin()
        .read_line(&mut raw_input)
        .expect("Failed to read from standard input.");

    let input = raw_input.trim().to_string();

    println!("Input the index of the word (N), which you want to extract:");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read from standard input.");

    let index: usize = index.trim().parse().expect("Failed to parse the input.");

    let extracted_word = extract_word(&input, index);

    println!("According to Ferris the Crab:\n\t{}", extracted_word);
}
