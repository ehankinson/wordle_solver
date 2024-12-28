mod wordle;

use wordle::get_words;
use wordle::final_info;
use wordle::final_word;
use wordle::get_letters;
use wordle::filter_words;
use std::io::{self, Write};
use wordle::grab_best_word;
use wordle::get_random_word;
use wordle::valid_word_prob;
use std::collections::HashSet;
use wordle::make_probabilities;


fn main() -> std::io::Result<()> {

    let mut input = String::new();

    loop {
        print!("Please enter 'easy' or 'hard': ");
        io::stdout().flush().unwrap(); // Ensure the prompt is printed before input
        
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let trimmed_input = input.trim(); // Trim whitespace

        if trimmed_input == "easy" || trimmed_input == "hard" {
            break; // Exit the loop when valid input is given
        } else {
            println!("Invalid input. Please enter either 'easy' or 'hard'.");
            input.clear(); // Clear the input string for the next attempt
        }
    }

    let mut final_guess = vec!['\0'; 5];

    let mut in_word: HashSet<char> = HashSet::new();

    let mut words = get_words(input.trim())?;

    let final_word = get_random_word(&words);

    let final_word_info = final_info(final_word);

    println!("There is a total of {:?} words to pick from", words.len());

    let letters = get_letters();

    let probabilites = make_probabilities(&words);

    let ordered_words = valid_word_prob(&words, probabilites);

    let best_word = grab_best_word(ordered_words);

    println!("The best word is {}", best_word);

    words = filter_words(&letters, words, &final_guess, &in_word);

    Ok(())
}
