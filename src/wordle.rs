use rand::Rng;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct LetterInfo {
    in_word: Option<bool>,
    double: bool,
    triple: bool,
    position: HashSet<usize>,
    not_position: HashSet<usize>,
}



pub fn get_letters() -> HashMap<char, LetterInfo> {
    let mut letters = HashMap::new();

    for ch in 'a'..='z' {
        letters.insert(
            ch,
            LetterInfo {
                in_word: None,
                double: true,
                triple: true,
                position: HashSet::new(),
                not_position: HashSet::new(),
            },
        );
    }

    return letters;
}



pub fn get_words(file_type: &str) -> std::io::Result<Vec<String>> {

    let file_path = if file_type == "easy" { "words/wordle_words.txt" } else { "words/valid_words.txt" };
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Ok(reader.lines().into_iter().map(|a| a.unwrap()).collect::<Vec<String>>())
    let words: Result<Vec<String>, io::Error> = reader.lines().collect();
    words

}



pub fn make_probabilities(words: &Vec<String>) -> HashMap<char, Vec<f64>> {
    let mut probs: HashMap<char, Vec<f64>> = HashMap::new();
    let divider: f64 = words.len() as f64;

    for word in words {

        for (i, char) in word.chars().enumerate() {

            if !probs.contains_key(&char) {
                probs.insert(char, vec![0.0; 5]);
            }

            if let Some(values) = probs.get_mut(&char) {
                values[i] += 1.0;
            }
        }
    }

    let mut final_probs: HashMap<char, Vec<f64>> = HashMap::new();

    for key in probs.keys() {

        if let Some(values) = probs.get(&key) {
            
            let mut probabilities = vec![0.0; 5];
            
            for (index, value) in values.iter().enumerate() {
                probabilities[index] = *value / divider;
            }

            final_probs.insert(*key, probabilities);
        }
    }

    final_probs

}



pub fn valid_word_prob(words: &Vec<String>, probabilites: HashMap<char, Vec<f64>>) -> Vec<(String, f64)> {
    let mut word_probabilites= Vec::new();

    for word in words.iter() {

        let mut chars: HashMap<char, f64> = HashMap::new();
        let mut word_prob: f64 = 0.0;

        for (i, char) in word.chars().enumerate() {

            if !chars.contains_key(&char) {
                chars.insert(char, 1.0);
            }
            else {
                if let Some(value) = chars.get_mut(&char) {
                    *value += 1.0;
                }
            }

            if let Some(prob_val) = probabilites.get(&char) {
                if let Some(char_val) = chars.get(&char) {
                    word_prob += prob_val[i] / char_val;
                }
            }
        }

        word_probabilites.push((word.clone(), word_prob));
    }

    word_probabilites.sort_by(|a, b| {
        b.1.partial_cmp(&a.1)
            .unwrap_or(std::cmp::Ordering::Equal) // safely handle the Option
    });

    word_probabilites
}



pub fn grab_best_word(mut ordered_words: Vec<(String, f64)>) -> String {
    // let guess_range = (ordered_words.len() as f64 * 0.1) as usize; 
    // let mut rng = rand::thread_rng();
    // let index = rng.gen_range(0..=guess_range); 

    // if ordered_words.len() <= index {
    //     return "hello world".to_string();
    // } 
    
    ordered_words.remove(0).0
}



pub fn filter_words(letters: &HashMap<char, LetterInfo>, words: Vec<String>, final_guess: &Vec<char>, in_word: &HashSet<char>) -> Vec<String> {
    let mut new_words = Vec::new();

    for word in words {

        let mut skip = false;
        let mut chars_map: HashMap<char, i8> = HashMap::new();
        let mut char_set: HashSet<char> = HashSet::new();

        for (i, char) in word.chars().enumerate() {

            *chars_map.entry(char).or_insert(0) += 1;
            char_set.insert(char);

            if final_guess[i] == '\0' {

                let letter_info = letters.get(&char).unwrap();

                if letter_info.in_word.is_none() { continue }

                let in_word = letter_info.in_word.unwrap();

                if !in_word {
                    skip = true;
                    break
                }

                else {
                    let pos = &letter_info.position;
                    let not_pos = &letter_info.not_position;
                    if pos.len() == 0 && not_pos.contains(&i) {
                        skip = true;
                        break
                    }
                }

            }

            else if final_guess[i] != char {
                skip = true;
                break;
            }
        }

        if skip { continue }

        if !in_word.is_subset(&char_set) { continue }

        for (key, value) in chars_map {
            let letter_info: &LetterInfo = letters.get(&key).unwrap();

            if let Some(in_word) = letter_info.in_word {
                if in_word && value > 1 {

                    if value == 2 && !letter_info.double {
                        skip = true;
                        break
                    }
    
                    if value == 3 && !letter_info.triple {
                        skip = true;
                        break;
                    }
                }
            }
            
        }
        
        if !skip { new_words.push(word) }

    }

    new_words
}



pub fn final_info(final_word: &String) -> HashMap<char, Vec<usize>> {
    let mut final_word_map: HashMap<char, Vec<usize>> = HashMap::new();

    for (i, char) in final_word.chars().enumerate() {
        if !final_word_map.contains_key(&char) {
            let indexs= vec![i];
            final_word_map.insert(char, indexs);
        }
        else {
            if let Some(indexs) = final_word_map.get_mut(&char) {
                indexs.push(i);
            }
        }
    }

    final_word_map
}



pub fn get_random_word(words: &Vec<String>) -> String {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..words.len());
    words[random_index].clone()
}



pub fn compare(final_word: &String, guess: String, letters: &mut HashMap<char, LetterInfo>, final_info: &HashMap<char, Vec<usize>>, final_guess: &mut Vec<char>, in_word: &mut HashSet<char>) -> bool {
    if &guess == final_word { return true }
    
    let mut is_word = true;
    let mut guess_letters: HashMap<char, Vec<usize>> = HashMap::new();

    for (i, char) in guess.chars().enumerate() {

        if !guess_letters.contains_key(&char) {
            let indexs= vec![i];
            guess_letters.insert(char, indexs);
        }
        else {
            if let Some(indexs) = guess_letters.get_mut(&char) {
                indexs.push(i);
            }
        }

        if final_info.contains_key(&char) {

            if let Some(letter_info) = letters.get_mut(&char) {
                letter_info.in_word = Some(true);

                if let Some(indexes) = final_info.get(&char) {

                    if indexes.contains(&i) {
                        letter_info.position.insert(i);
                        final_guess[i] = char;
                        in_word.insert(char);
                    }
                    else {
                        letter_info.not_position.insert(i);
                        in_word.insert(char);
                        is_word = false;
                    }
                }
            }
        }
        else {
            if let Some(letter_info) = letters.get_mut(&char) {
                letter_info.in_word = Some(false);
                is_word = false;
            }
        }
    }

    for (key, value) in guess_letters {

        if final_info.contains_key(&key) {
            let len = value.len();
            let final_len = final_info.len();

            let letter_info = letters.get_mut(&key).unwrap();

            if len > final_len {
                letter_info.triple = false;
                if len == 2 {
                    letter_info.double = false;
                }
            }
            else if len == 3 {
                letter_info.double = false;
            }

        }
    }
        
    is_word
}