use std::{fs, io};
use rand::Rng;

// this is my function that I use to check if there are any dashes in the
// hidden word. If there are dashes, meaning the word is not guessed completely
// then it will return true but if there are no dashes then it will return false
fn check_word(word: &str) -> bool {
    word.contains('-')
}

fn main() {

    //prompt the user to play the game
    println!("Welcome to the game of Hangman!");
    println!("Enter letters one by one to guess the word!");

    // taking the text file and reading it into a string
    let all_words = fs::read_to_string("src/words.txt")
        .expect("Failed to read the file");

    // creating a vector of words from the string containing the .txt file
    let words: Vec<&str> = all_words.lines()
        .collect();

    // getting a random index so I can use it to grab a random word
    // the rand library is used based on the dependency in the .toml file
    let random_index = rand::thread_rng()
        .gen_range(0..words.len());
    
    // here is where I'm using the random index to get a random word in the 
    // vector of all the words in the text file
    let hangman_word = words[random_index];
    
    // ===================================================================
    // I used this for testing
    //println!("The random number is: {}", random_index);
    // UN-COMMENT THIS TO KNOW THE HIDDEN WORD
    //println!("The random word is: {}", hangman_word);
    // ===================================================================

    // I want to display the length of the word for the users to guess
    // so I have to copy the real word and hide the characters but
    // show how many characters there are.
    let hidden_word_length = hangman_word.len();
    let mut hidden_word = String::new();

    // converting the hangman_word into a word with as many dashes as there
    // are letters in the word.
    for _ in 0..hidden_word_length {
        hidden_word.push('-');
    }
    
    // displaying what the user has so far
    println!("{}", hidden_word);
    println!("");

    // This is where I'm starting the game
    println!("Enter a letter or type 'exit' to quit the game: ");

    // I read in the users input into a string and then slice it to remove the 
    // whitespace but convert it back into a string.
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input = input.trim().to_string();

    // this is where the game is looping to receive new letters for guesses
    // or if they want to exit then they can choose exit.
    while check_word(&hidden_word) {

        if input == "exit" {
            break;
        }

        // I use this to decide whether or not to tell the user the letter was 
        // in the word.
        let mut found = false;

        // this is where the magic happens. I take the users guess and compare it to each letter in the
        // hangman word. I iterate through by index of the word but then if I find a match, change the dash
        // in the hidden word and replace it with the letter that was guessed
        for (i, c) in hangman_word.chars().enumerate() {
            if c.to_string() == input {
                hidden_word.replace_range(i..=i, &c.to_string());
                found = true;
            }
        }
        
        // if the letter guessed isn't in the word it will tell display this and iterate through again.
        if !found {
            println!("{} isn't in the word! Try again!", input);
        }

        // displaying what the user has guessed so far in the word
        println!("{}", hidden_word);

        // I had to include this check because after successfully winning the game it would
        // still prompt the user to enter another letter. This stops that from happening.
        if !check_word(&hidden_word) {
            break;
        } else {
            // this is making sure that the user's previous input
            // isn't still in the input variable.
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            input = input.trim().to_string();
        }
    }
    
    // as the user guesses correct letters the dashes get replaced with the actual
    // letters. So if the hidden word == the hangman_word that means they've finised
    // the game successfully without exiting.
    if hidden_word == hangman_word {
        println!("You got it! The word was: {}", hangman_word);
    } else {
        println!("Thanks for playing! See you soon!");
    }
}
