// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
//use std::io::Write;
use std::collections::HashMap;
use std::convert::TryInto;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

// Convert vec of chars into string
fn vec_char_to_string(chars: &Vec<char>) -> String{
    chars.into_iter().collect()
}

// struct for single char
struct Char{
    count: u32,         // count of char in word
    guessed_times: u32, // time of char guessed by user
    idxs: Vec<u32>,     // vec of index of char in word
}

// build HashMap of given secrect word
fn hash_words(secret_word_chars: &Vec<char>) -> HashMap<char, Char>{
    let mut hash_map = HashMap::new();
    for (idx, ch) in secret_word_chars.iter().enumerate(){
        let curr = hash_map.entry(*ch).or_insert(Char{
            count: 0,
            guessed_times: 0,
            idxs: Vec::new(),
        });
        (*curr).count += 1;
        (*curr).idxs.push(idx.try_into().unwrap());
    }
    hash_map
}

// Convert input to char
fn input_to_char() -> char{
    println!("Please Enter a Letter: ");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess) 
        .expect("Failed to read line");
    
    let guess: char = guess.trim().parse().expect("Please type in a letter!");

    guess
}

enum Index {
    Yes(u32),
    No,
}

fn guess_in_word(guess: char, hash_map: &mut HashMap<char, Char>) -> Index{
    match hash_map.get_mut(&guess){
        Some(char_of_guess) => {
            // revial one of the char, 
            // increase guessed_times by one
            let curr_idx;
            // get index_of_guess by index of idxs
            match char_of_guess.idxs.get(char_of_guess.guessed_times as usize){
                Some(idx) => curr_idx = idx,
                None => {
                    return Index::No;
                }
            }
            char_of_guess.guessed_times += 1;
            Index::Yes(*curr_idx)
        }
        None => Index::No,
    }
}

// game
fn hangman(secret_word_chars: &Vec<char>){
    let mut pass = false;
    let mut num_incorrect_guess = 0;
    let mut hash_secret_word = hash_words(&secret_word_chars);
    let mut curr_word_chars = vec!['-'; secret_word_chars.len()];
    let mut curr_guessed_chars = Vec::new();
    
    while num_incorrect_guess < NUM_INCORRECT_GUESSES{
        println!(
            "The word so far is {}", vec_char_to_string(&curr_word_chars)
        );
        println!(
            "You have guessed the following letters: {}",
            vec_char_to_string(&curr_guessed_chars)
        );

        let curr_char = input_to_char();

        if curr_char.is_alphabetic(){
            match guess_in_word(curr_char, &mut hash_secret_word){
                Index::Yes(idx) => {
                    curr_word_chars[idx as usize] = curr_char;
                    if curr_word_chars == *secret_word_chars{
                        pass = true;
                        break;
                    }
                }
                Index::No =>{
                    num_incorrect_guess += 1;
                    println!("Sorry, the letter is wrong.");
                }
            }
            curr_guessed_chars.push(curr_char)
        } else {
            panic!("Not supported input type!");
        }
        println!("------------ Round ends ---------------\n")
    }

    if pass {
        println!(
            "Congratulations you guessed the secret word: {}!",
            vec_char_to_string(&curr_word_chars)
        );
    } else {
        println!(
            "Sorry you ran out of guesses!"
        );
    }
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);
    hangman(&secret_word_chars);
}


