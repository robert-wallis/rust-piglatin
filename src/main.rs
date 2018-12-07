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
    let mut words:Vec<&str> = vec![];
    let mut start = 0;
    for (i, c) in line.char_indices() {
        if !c.is_alphabetic() {
            if start != i {
                words.push(&line[start..i]);
            }
            if !c.is_whitespace() {
                words.push(&line[i..i+1]);
            }
            start = i+1;
        }
    }
    println!("mid:{:#?}", words);
    let converted =
        words
        .iter()
        .map(|w| ay_word(w))
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
    let first_letter = chars[0];
    match first_letter {
        'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U' | 'y' | 'Y' => {
            ay_combine(word, "hay")
        },
        _ => {
            if !first_letter.is_alphabetic() {
                String::from(word)
            } else {
                for c in chars.iter().skip(1) {
                    front.push(*c);
                }
                let ay = format!("{}{}", first_letter, "ay");
                ay_combine(&front, &ay)
            }
        }
    }
}

fn ay_combine(front: &str, ay: &str) -> String {
    format!("{}-{}", front, ay)
}
