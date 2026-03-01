// Assignment 2: Word Frequency Counter
// Create a program that:

// Takes a string of text as input
// Splits the text into words (space as separator) // text.split_whitespace().collect();
// Counts the frequency of each word
// Returns the word with the highest frequency and its count


fn most_frequent_word(text: &str) -> (String, usize) {

    // split text
    let words: Vec<&str> = text.split_whitespace().collect();
    
    //(max_word, max_count) // return tuple
    let mut max_word = String::new(); // maximum word
    let mut max_count = 0; //count

    // loop, indexing through each word
    for i in 0..words.len() {
        let current_word = words[i]; // current word counting

        let mut count = 0; //counter for current_word

        // loop, compare current_word with other words
        for x in 0..words.len() {
            if words[x] == current_word {
                count += 1; // increasing count if match
            }
        }
        //update max word and count
        if count > max_count {
            max_count = count;
            max_word = current_word.to_string();
        }
    }
    (max_word, max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}