/*** 

Word Jumble 

Player is giving a jumbled word and asked to unscramble it.

***/


use std::vec;
use std::io;
use std::io::Write;
use rand::Rng;
use std::collections::HashMap;
use serde_json;
use reqwest;


fn clear_screen() {
    print!("{}[2J{}[H", 27 as char, 27 as char);
    std::io::stdout().flush().unwrap();
}

fn get_words() -> Vec<String> {
    let url = "https://raw.githubusercontent.com/dwyl/english-words/master/words_dictionary.json";
    let response = reqwest::blocking::get(url).expect("Failed to fetch words dictionary");
    let words_dict: HashMap<String, u32> = serde_json::from_str(response.text().unwrap().as_str()).unwrap();
    let words: Vec<String> = words_dict.keys().cloned().collect();
    words
}

fn insert_rand_index(used_indeces: &mut Vec<i32>, limit: usize) -> i32 {
    let rand_index = rand::thread_rng().gen_range(0..limit);
    let rand_index = rand_index as i32;
    if used_indeces.contains(&rand_index) {
        return insert_rand_index(used_indeces, limit);
    }

    used_indeces.push(rand_index);
    
    rand_index
}

fn scramble_word(word: &String) -> String {
    let original_word_array: Vec<char> = word.chars().collect();
    let mut word_array = original_word_array.clone();
    let mut used_indeces: Vec<i32> = vec![];

    for i in 0..word.len() {
        let rand_index = insert_rand_index(&mut used_indeces, word.len());
        word_array[i] = original_word_array[rand_index as usize];
    }

    word_array.into_iter().collect()
}


fn play(words: &Vec<String>) {
    let rand_index = rand::thread_rng().gen_range(0..words.len());
    let word = &words[rand_index];
    let scrambled_word = scramble_word(word);

    println!("Your scrambled word is: {}", scrambled_word);

    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_input).expect("Invalid input");
    user_input = user_input.trim().to_string();

    if &user_input == word {
        println!("You win!")
    } else {
        println!("Nope! The correct word was:\n{}", word)
    }

}


fn prompt_exit() {
    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_input).expect("Invalid input");
    user_input = user_input.trim().to_string();
    if user_input == "?exit" {
        std::process::exit(0);
    }
}

fn continuous_play(words: Vec<String>) {
    loop {
        clear_screen();
        play(&words);
        prompt_exit();
    }
}

fn main() {
    clear_screen();
    println!("Loading words...");
    let words = get_words();
    continuous_play(words);
}
