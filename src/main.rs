// Copyright (C) 2018 Robert A. Wallis
// Convert strings to pig latin.
//      The first consonant of each word is moved to the end of the word
//      and “ay” is added, so “first” becomes “irst-fay.”
//      Words that start with a vowel have “hay” added to the end instead
//      (“apple” becomes “apple-hay”).
//      Keep in mind the details about UTF-8 encoding!
// - https://doc.rust-lang.org/book/ch08-03-hash-maps.html

use std::io::{self};

fn main() {
    loop {
        let input = read_line();
        if input.is_empty() {
            return;
        }
        ay_line(&input);
    }
}

fn read_line() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Err(err) => {
            eprintln!("{}", err);
            input
        },
        Ok(_size) => input,
    }
}

fn ay_line(line: &str) {
    let line = line.trim();
    if line.is_empty() {
        println!();
        return;
    }
    let converted = line
        .split_whitespace()
        .map(ay_word)
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", converted);
}

fn ay_word(word: &str) -> String {
    let chars = word.chars().collect::<Vec<char>>();
    let mut front = String::new();
    if chars.is_empty() {
        return front;
    }
    match chars[0] {
        'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U' | 'y' | 'Y' => {
            ay_combine(word, "hay")
        },
        _ => {
            for c in chars.iter().skip(1) {
                front.push(*c);
            }
            let ay = format!("{}{}", chars[0], "ay");
            ay_combine(&front, &ay)
        }
    }
}

fn ay_combine(front: &str, ay: &str) -> String {
    let mut word = String::new();
    word.push_str(front);
    word.push('-');
    word.push_str(ay);
    word
}
