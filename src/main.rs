// Copyright (C) 2018 Robert A. Wallis
// Convert strings to pig latin.
//      The first consonant of each word is moved to the end of the word
//      and “ay” is added, so “first” becomes “irst-fay.”
//      Words that start with a vowel have “hay” added to the end instead
//      (“apple” becomes “apple-hay”).
//      Keep in mind the details about UTF-8 encoding!
// - https://doc.rust-lang.org/book/ch08-03-hash-maps.html

use std::io;

fn main() {
    loop {
        let input = read_line();
        if input.is_empty() {
            return;
        }
        let line = ay_line(&input);
        println!("{}", &line);
    }
}

/// Get a single line from stdin.
/// If there's an error with stdin, prints error to stderr and returns empty string.
/// EOF returns empty string, "\r\n" or "\n" is not an empty string, so only EOF or error returns empty string.
fn read_line() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Err(err) => {
            eprintln!("{}", err);
            input
        }
        Ok(_size) => input,
    }
}

/// Given a line, translates it into piglatin.
fn ay_line(line: &str) -> String {
    let line = line.trim();
    if line.is_empty() {
        return String::from("");
    }
    tokenize(line)
        .iter()
        .map(|w| ay_word(w))
        .collect::<Vec<String>>()
        .join("")
}

/// Given a word, figures out the piglatin for it.
pub fn ay_word(word: &str) -> String {
    let chars = word.chars().collect::<Vec<char>>();
    let mut front = String::new();
    if chars.is_empty() {
        return front;
    }
    let first_letter = chars[0];
    match first_letter {
        'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U' => {
            ay_combine(word, "hay")
        }
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

/// Adds a dash between the front and the ay part of the word, unless one is missing.
fn ay_combine(front: &str, ay: &str) -> String {
    let mut list = Vec::new();
    if !front.is_empty() {
        list.push(front);
    }
    if !ay.is_empty() {
        list.push(ay);
    }
    list.join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ay_line_happy() {
        assert_eq!("irst-fay apple-hay", ay_line("first apple"));
        assert_eq!(
            "o-day ou-yay peak-say ig-pay atin-lay?",
            ay_line("do you speak pig latin?")
        );
        assert!(ay_line("\n").is_empty());
    }

    #[test]
    #[ignore]
    fn ay_word_todo() {
        assert_eq!("opping-shay", ay_word("shopping"));
        assert_eq!("Eat-nay", ay_word("Neat"));
    }

    #[test]
    fn ay_word_happy() {
        assert_eq!("irst-fay", ay_word("first"));
        assert_eq!("apple-hay", ay_word("apple"));
        assert_eq!("eato-nay", ay_word("neato"));
        assert_eq!("urrito-bay", ay_word("burrito"));
        assert_eq!("es-yay", ay_word("yes"));
    }

    #[test]
    fn ay_word_non_words() {
        assert_eq!("", ay_word(""));
        assert_eq!(" ", ay_word(" "));
        assert_eq!("  ", ay_word("  "));
        assert_eq!("1", ay_word("1"));
        assert_eq!("22", ay_word("22"));
        assert_eq!("1.21", ay_word("1.21"));
        assert_eq!(".", ay_word("."));
        assert_eq!(",", ay_word(","));
        assert_eq!("-", ay_word("-"));
        assert_eq!("1-1", ay_word("1-1"));
    }

    #[test]
    fn ay_combine_happy() {
        assert_eq!(
            "eato-nay",
            ay_combine(&String::from("eato"), &String::from("nay"))
        );
        assert_eq!(
            "urrito-bay",
            ay_combine(&String::from("urrito"), &String::from("bay"))
        );
    }

    #[test]
    fn ay_combine_empty() {
        assert_eq!("beep", ay_combine(&String::from(""), &String::from("beep")));
        assert_eq!("boop", ay_combine(&String::from("boop"), &String::from("")));
    }
}

/// Given a string, splits it into tokens, keeping the characters.
fn tokenize(line: &str) -> Vec<&str> {
    let mut words: Vec<&str> = vec![];
    let mut start = 0;
    for (i, c) in line.char_indices() {
        if !c.is_alphabetic() {
            if start != i {
                words.push(&line[start..i]);
            }
            words.push(&line[i..=i]);
            start = i + 1;
        }
    }
    if start < line.len() {
        words.push(&line[start..]);
    }
    words
}