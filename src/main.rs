// #![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(unused_assignments)]
// #![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unreachable_code)]

extern crate argparse;

use std::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fmt;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use argparse::{ArgumentParser, Store};


#[derive(Debug)]
struct MyErr {
    msg: String
}

impl MyErr {
    fn new(message: String) -> Self {
        MyErr {
            msg: message
        }
    }
}

#[derive(Clone, Debug)]
struct Word {
    data: String,
    map: HashMap<u8, HashSet<usize>>,
}

impl Word {
    fn new(word_string: &str) -> Self {
        let mut map_: HashMap<u8, HashSet<usize>> = HashMap::new();
        for (idx, letter) in word_string.chars().enumerate() {
            let indices = map_.entry(letter as u8).or_insert(HashSet::new());
            indices.insert(idx);
        }
        Word {
            map: map_,
            data: word_string.to_string(),
        }
    }
}

fn get_word_bank(fname: &str) -> Vec<Word> {
    let path = Path::new(fname);
    let file = match File::open(path) {
        Ok(file) => file,
        Err(err) => panic!(
            "Could not open {display} because {err}",
            display = path.display(),
            err = err
        ),
    };

    let bf = BufReader::new(file);

    let mut word_bank: Vec<Word> = Vec::new();
    for line in bf.lines() {
        word_bank.push(Word::new(&line.unwrap()));
    }
    word_bank
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Color {
    GRAY,
    YELLOW,
    GREEN,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::GRAY => write!(f, "{}", '\u{2B1B}'),
            Color::YELLOW => write!(f, "{}", '\u{1F7E8}'),
            Color::GREEN => write!(f, "{}", '\u{1F7E9}'),
        }
    }
}

const WORD_LENGTH: usize = 5;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Filter {
    colors: [Color; WORD_LENGTH],
}
impl Filter {
    fn new() -> Self {
        Filter {
            colors: [Color::GRAY, Color::GRAY, Color::GRAY, Color::GRAY, Color::GRAY],
        }
    }
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}{}{}", self.colors[0], self.colors[1], self.colors[2], self.colors[3], self.colors[4])
    }
}

fn compute_filter(query: &Word, secret: &Word) -> Filter {
    let mut filter = Filter::new();

    for i in 0..query.data.len() {
        // Skip grays
        let letter = query.data.as_bytes()[i];
        if !secret.map.contains_key(&letter) {
            continue;
        }

        // Greens
        let query_letter_indices = &query.map[&letter];
        let secret_letter_indices = &secret.map[&letter];
        let green_indices: HashSet<&usize> = query_letter_indices
            .intersection(secret_letter_indices)
            .collect();
        for idx in &green_indices {
            filter.colors[**idx] = Color::GREEN;
        }

        // Yellows
        let num_yellows =
            min(query_letter_indices.len(), secret_letter_indices.len()) - green_indices.len();

        // query index set minus green index set = (potential) yellow index set
        let mut yellow_indices = query_letter_indices.clone();
        for idx in &green_indices {
            yellow_indices.remove(idx);
        }
        let mut yellow_indices_vec = Vec::from_iter(&yellow_indices);
        yellow_indices_vec.sort();

        // Mark yellow indices as yellow.
        // Mark earlier instances as yellow over later instances in the event of duplicate letters
        for i in 0..num_yellows {
            filter.colors[*yellow_indices_vec[i]] = Color::YELLOW;
        }
    }

    filter
}

fn compute_filters_to_secret_candidates_for_query( query: &Word, secret_candidates: &Vec<Word>) -> HashMap<Filter, Vec<Word>> {
    let mut filters_to_secret_candidates: HashMap<Filter, Vec<Word>> = HashMap::new();
    for secret in secret_candidates {
        let filter = compute_filter(&query, &secret);
        let mapped_candidates = filters_to_secret_candidates.get_mut(&filter);
        match mapped_candidates {
            Some(p) => {
                p.push(secret.clone());
            },
            None => match filters_to_secret_candidates.insert(filter.clone(), vec![secret.clone()]) {
                Some(_) => (),
                None => ()
            }
        }
    }

    filters_to_secret_candidates
}

fn compute_hashmap_cost(filters_to_secret_candidates_for_query: &HashMap<Filter, Vec<Word>>) -> u32 {
    let mut cost: u32 = 0;

    for (_, secret_candidates_from_filter) in filters_to_secret_candidates_for_query.iter() {
        // cost is worst-case performance
        if secret_candidates_from_filter.len() as u32 > cost {
            cost = secret_candidates_from_filter.len() as u32
        }
    }

    cost
}

fn compute_query_cost(query: &Word, secret_candidates: &Vec<Word>) -> (u32, HashMap<Filter, Vec<Word>>) {
    let filters_to_secret_candidates_for_query =
        compute_filters_to_secret_candidates_for_query(&query, &secret_candidates);
    let hashmap_cost = compute_hashmap_cost(&filters_to_secret_candidates_for_query);
    (hashmap_cost, filters_to_secret_candidates_for_query)
}

fn get_filter_from_input() -> Filter {
    let mut filter = Filter::new();
    loop {
        // Get input from user
        let mut input = String::new();
        println!("Enter colors with spaces in between: ");

        // Split input by whitespace
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        };
        let input_vec: Vec<&str> = input.split_whitespace().collect();

        // Check number of colors inputted
        if input_vec.len() != WORD_LENGTH {
            println!("Expected {} colors, got {}", WORD_LENGTH, input_vec.len());
            continue;
        }

        // Check that all colors are valid:
        let mut colors_valid: bool = true;
        for (idx, word) in input_vec.iter().enumerate() {
            match &*word.to_lowercase() {
                "gray" => filter.colors[idx] = Color::GRAY,
                "yellow" => filter.colors[idx] = Color::YELLOW,
                "green" => filter.colors[idx] = Color::GREEN,
                _ => {
                    println!("Unexpected color: {}", &*word.to_lowercase());
                    println!("Colors can be gray, yellow, or green");
                    colors_valid = false;
                    break;
                }
            }
        }

        // At this point, we have `WORD_LENGTH` valid colors, so exit out of loop
        if colors_valid {
            break;
        }
    }

    filter
}

fn compute_best_query(word_bank: &Vec<Word>, secret_candidates: &Vec<Word>) -> Word {
    let mut minimum_cost_query: Word = word_bank[0].clone();
    let mut minimum_cost: u32 = u32::MAX;

    for query in word_bank {
        let (cost, _) = compute_query_cost(query, secret_candidates);
        if cost < minimum_cost {
            minimum_cost = cost;
            minimum_cost_query = query.clone();
        }
    }
    minimum_cost_query
}

fn write_result_to_file(query: &String, guesses: &i32) {
    // Create results.txt if it does not exist
    if !Path::new("results.txt").exists() {
        let _ = match File::create("results.txt") {
            Ok(_) => (),
            Err(_) => panic!("Could not create results.txt"),
        };
    }

    // Write to results.txt
    let mut file = OpenOptions::new().append(true).open("results.txt").unwrap();
    if let Err(e) = writeln!(file, "{} {}", query, guesses) {
        eprintln!("Couldn't write to file {}", e);
    }
}

fn main() {
    let mut arg_path_queries: String = "queries.txt".to_string();
    let mut arg_path_solutions: String = "solutions.txt".to_string();
    let mut arg_first_guess: String = "aesir".to_string();
    let mut arg_known_secret: String = "".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Solve Wordle.");
        ap.refer(&mut arg_first_guess).add_option(&["-g", "--guess"], Store, "First guess. If unspecified, compute best first guess based on word bank and cost function, and exit program.");
        ap.refer(&mut arg_path_queries).add_option(&["--queries"], Store, "Path to query word bank");
        ap.refer(&mut arg_known_secret).add_option(&["-s", "--secret"], Store, "Simulate program on a known secret.");
        ap.refer(&mut arg_path_solutions).add_option(&["--solutions"], Store, "Path to solution word bank");
        ap.parse_args_or_exit();
    }

    // TODO: Command line argument to do filter generation test
    // Uncomment to test what filter will be generated by a query and a secret
    // Should be yellow, grey, green, yellow, green

    // println!("final filter: {}", compute_filter(&Word::new(&String::from("oooll")), &Word::new(&String::from("llool"))));
    // return;

    // Main application
    // TODO: Check that word bank exists by returning a Result from get_word_bank
    let word_bank = get_word_bank(&arg_path_queries);
    let mut secret_candidates = get_word_bank(&arg_path_solutions);

    secret_candidates.sort_by(|a, b| a.data.cmp(&b.data));

    // Best first word is precomputed to save time.
    let mut best_query: Word;
    if arg_first_guess.len() == 0 {
        println!("Computing best starting word from {}...", arg_path_queries);
        best_query = compute_best_query(&word_bank, &secret_candidates);
        println!("{}", best_query.data);
        return;
    }
    else if arg_first_guess.len() == WORD_LENGTH {
        best_query = Word::new(&arg_first_guess.to_string());
    }
    else {
        println!("Guess must be {} letters long", WORD_LENGTH);
        return;
    }

    let mut best_query_filters_to_secret_candidates: HashMap<Filter, Vec<Word>>;

    let mut guesses = 1;
    let mut filter: Filter;

    loop {
        // If a guess has already been made, need to compute the next best guess
        if guesses > 1 {
            best_query = compute_best_query(&word_bank, &secret_candidates);
        }

        // Supply best guess to user
        best_query_filters_to_secret_candidates = compute_query_cost(&best_query ,&secret_candidates).1;
        println!("Best guess: {}", best_query.data);

        if arg_known_secret.len() == WORD_LENGTH {
            // Calculate filter automatically when secret word is known (testing only)
            filter = compute_filter(&best_query, &Word::new(&String::from(&arg_known_secret)));
        }
        else {
            // Get filter from user
            filter = get_filter_from_input();
        }

        // Print filter to user
        if filter.colors == [Color::GREEN; 5] {
            println!("FOUND: {} in {} guess{}", best_query.data, guesses, if guesses != 1 {"es"} else {""});
            break;
        }

        // secret_candidates is the list of possible solutions that the filter maps to
        secret_candidates = match best_query_filters_to_secret_candidates.get(&filter) {
            Some(candidates) => {
                candidates.clone()
            },
            None => {
                println!("Couldn't find a word!");
                return;
            }
        };

        // Check if secret_candidates contains only one word
        match secret_candidates.len() {
            0 => {
                println!("Couldn't find a word!");
                break;
            },
            1 => {
                guesses += 1;
                println!("FOUND: {} in {} guess{}", &secret_candidates[0].data, guesses, if guesses > 1 { "es" } else { "" });
                break;
            },
            _ => {
                guesses += 1;
            },
        }
    }
}
