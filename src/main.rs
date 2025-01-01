mod wordle;

use std::time::Instant;
use std::io::{self, Write};
use std::collections::HashSet;

use wordle::compare;
use wordle::get_words;
use wordle::final_info;
use wordle::get_letters;
use wordle::filter_words;
use wordle::grab_best_word;
use wordle::get_random_word;
use wordle::valid_word_prob;
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

    let mut count = 0;
    let sim = 1000;

    let start = Instant::now();

    for _ in 0..sim {

        let mut final_guess = vec!['\0'; 5];

        let mut in_word: HashSet<char> = HashSet::new();

        let mut words = get_words(input.trim())?;

        let final_word = get_random_word(&words);

        // println!("The word to be guessed is: {:?}", final_word);

        let final_word_info = final_info(&final_word);

        // println!("There is a total of {:?} words to pick from", words.len());

        let mut letters = get_letters();

        for _ in 0..6 {

            let probabilites = make_probabilities(&words);

            let ordered_words = valid_word_prob(&words, probabilites);

            let best_word = grab_best_word(ordered_words);

            // println!("The best word is {}", best_word);
            
            let is_word = compare(&final_word, best_word, &mut letters,&final_word_info, &mut final_guess, &mut in_word);

            if is_word {
                // println!("You guessed the correct word");
                // let last_guess: String = final_guess.into_iter().collect();
                // println!("The final word was: {:}", last_guess);
                count += 1;
                break;
            }

            words = filter_words(&letters, words, &final_guess, &in_word);

            // println!("There are now {:?} words to choose from.", words.len());
        }
       
    }
    let duration = start.elapsed();
    println!("The wordle has a success rate of {:.2}%", (count as f64 / sim as f64) * 100.0);
    println!("Time taken: {:.2?}", duration);

    Ok(())
    
}
