/*
Author: Chris Robertson https://github.com/electronicsleep
Purpose: Simple FlashCard app in Rust
Released under the MIT license
*/

use rand::Rng;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn read_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No file found: flash_cards.txt");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let mut num_checks = 0;
    let total_checks = 10;

    let mut questions = Vec::new();
    let mut answers = Vec::new();
    println!("-> Import flash card file");

    let lines = read_file("src/flash_cards.txt");
    for line in lines {
        if line.contains("Q. ") {
            questions.push(line.replace("Q. ", ""));
        //println!("Added question");
        } else if line.contains("A. ") {
            answers.push(line.replace("A. ", ""));
        }
    }

    let num_cards = questions.len();
    println!("num cards {}", num_cards);

    let mut score = 0;
    println!("-> Starting FlashCards");

    while num_checks < total_checks {
        num_checks = num_checks + 1;

        let current = rand::thread_rng().gen_range(0, num_cards);
        println!("-> FlashCards Card {}", current);

        let mut line = String::new();
        println!("Question: {}", questions[current]);
        let line_len = std::io::stdin().read_line(&mut line).unwrap();
        println!("line: {}", line);
        println!("line_len: {}", line_len);

        let mut answer = line.to_string();
        let len = answer.len();
        answer.truncate(len - 1);
        println!("Check Answer |{}| |{}|", answer, answers[current]);
        if answer == answers[current] {
            println!("Correct");
            score = score + 1;
        } else {
            println!("Incorrect");
            score = score - 1;
        }
        println!(
            "-> Score: {} Run: {} of {}",
            score, num_checks, total_checks
        );
    }
}
