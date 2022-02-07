// #![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(unused_assignments)]
// #![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unreachable_code)]

use std::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Clone)]
struct Word {
    map: HashMap<u8, HashSet<usize>>,
    data: String,
}

impl Word {
    fn new(word_string: &String) -> Self {
        let mut map_: HashMap<u8, HashSet<usize>> = HashMap::new();
        for (idx, letter) in word_string.chars().enumerate() {
            let indices = map_.entry(letter as u8).or_insert(HashSet::new());
            indices.insert(idx);
        }
        Word {
            map: map_,
            data: word_string.clone(),
        }
    }
}

fn get_word_bank(fname: &str) -> Vec<Word> {
    let path = Path::new(fname);
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => panic!(
            "Could not open {display} because {err}",
            display = path.display(),
            err = err
        ),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(err) => panic!(
            "Could not read {display} because {err}",
            display = path.display(),
            err = err
        ),
    };

    let newline: String = match env::consts::OS {
        "windows" => String::from("\r\n"),
        _ => String::from("\n"),
    };

    let word_bank: Vec<&str> = contents.split(&newline).collect();
    let mut word_bank_word: Vec<Word> = Vec::new();
    for word in word_bank {
        let w: Word = Word::new(&String::from(word));
        word_bank_word.push(w);
    }
    word_bank_word
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
            Color::GRAY => write!(f, "{}", "\u{2B1B}"),
            Color::YELLOW => write!(f, "{}", '\u{1F7E8}'),
            Color::GREEN => write!(f, "{}", '\u{1F7E9}'),
        }
    }
}

const WORD_LENGTH: usize = 5;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Filter {
    colors: [Color; WORD_LENGTH],
}
impl Filter {
    fn new() -> Self {
        Filter {
            colors: [Color::GRAY; WORD_LENGTH],
        }
    }
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Scale with WORD_LENGTH
        write!(
            f,
            "{}{}{}{}{}",
            self.colors[0], self.colors[1], self.colors[2], self.colors[3], self.colors[4]
        )
    }
}

// TODO: Check that query.data and secret.data have length equal to WORD_LENGTH
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

fn compute_filters_to_secret_candidates_for_query(
    query: &Word,
    secret_candidates: &Vec<Word>,
) -> HashMap<Filter, Vec<Word>> {
    let mut filters_to_secret_candidates: HashMap<Filter, Vec<Word>> = HashMap::new();
    for secret in secret_candidates {
        let filter = compute_filter(&query, &secret);
        filters_to_secret_candidates
            .entry(filter)
            .or_insert(Vec::new());
        filters_to_secret_candidates
            .get_mut(&filter)
            .unwrap()
            .push(secret.clone())
    }
    filters_to_secret_candidates
}

fn compute_hashmap_cost(
    filters_to_secret_candidates_for_query: &HashMap<Filter, Vec<Word>>,
) -> usize {
    let mut max_count: usize = 0;

    for (_, secret_candidates_from_filter) in filters_to_secret_candidates_for_query.iter() {
        // cost is worst-case performance
        if secret_candidates_from_filter.len() > max_count {
            max_count = secret_candidates_from_filter.len();
        }
    }

    max_count
}

fn compute_query_cost(query: &Word, word_bank: &Vec<Word>) -> (usize, HashMap<Filter, Vec<Word>>) {
    let filters_to_secret_candidates_for_query =
        compute_filters_to_secret_candidates_for_query(&query, &word_bank);
    let hashmap_cost = compute_hashmap_cost(&filters_to_secret_candidates_for_query);
    (hashmap_cost, filters_to_secret_candidates_for_query)
}

/*
* TODO: Check that input is exactly WORD_LENGTH words
* TODO: CHeck that all words in input are either gray, yellow, or green
*/
fn get_filter_from_input() -> Filter {
    let mut input = String::new();
    println!("Enter lowercase colors with spaces in between: ");
    let _ = std::io::stdin().read_line(&mut input).unwrap();
    let input_vec: Vec<&str> = input.split_whitespace().collect();
    let mut filter = Filter::new();
    for (idx, word) in input_vec.iter().enumerate() {
        match &*word.to_lowercase() {
            "gray" => filter.colors[idx] = Color::GRAY,
            "yellow" => filter.colors[idx] = Color::YELLOW,
            "green" => filter.colors[idx] = Color::GREEN,
            _ => (),
        }
    }

    filter
}

fn compute_best_query(word_bank: &Vec<Word>) -> Word {
    let mut minimum_cost_query: Word = word_bank[0].clone();
    let mut minimum_cost: usize = usize::MAX;

    for query in word_bank {
        let (cost, _) = compute_query_cost(query, word_bank);
        if cost < minimum_cost {
            minimum_cost = cost;
            minimum_cost_query = query.clone();
        }
    }
    minimum_cost_query
}

fn main() {
    // Uncomment to test what filter will be generated by a query and a secret
    // Should be yellow, grey, green, yellow, green
    // println!(
    //     "final filter: {}",
    //     compute_filter(
    //         &Word::new(&String::from("oooll")),
    //         &Word::new(&String::from("llool"))
    //     )
    // );
    // return;

    // Main application
    // TODO: Command line argument to get path to work bank
    let mut word_bank = get_word_bank("sgb-words.txt");

    /*
     * Best first word is precomputed to save time.
     * To compute best first word, uncomment the next two lines.
     * When the first best word has been found, comment the next two lines and type it
     * into the Word constructor for best_query below.
     */
    // TODO: Command line argument to decide whether or not to compute best word
    // println!("{}", compute_best_query(&word_bank).data);
    // return;

    let mut best_query = Word::new(&String::from("aloes"));
    let mut best_query_filters_to_secret_candidates: HashMap<Filter, Vec<Word>>;

    let mut guesses = 0;
    let mut filter: Filter;

    loop {
        // If a guess has already been made, need to compute the next best guess
        if guesses > 0 {
            best_query = compute_best_query(&word_bank);
        }

        // Supply best guess to user
        let (_, b) = compute_query_cost(&best_query, &word_bank);
        best_query_filters_to_secret_candidates = b;
        println!("Best guess: {}", best_query.data);

        // TODO: Command line argument to decide whether to simulate with secret or play real game
        filter = get_filter_from_input(); // Get filter from user
        // filter = compute_filter(&best_query, &Word::new(&String::from("skill"))); // Calculate filter automatically when secret word is known (testing only)
        println!("Filter received: {:?}", filter);

        // word_bank is the list of words that the filter maps to
        word_bank = best_query_filters_to_secret_candidates
            .get(&filter)
            .unwrap()
            .clone();

        // Check if word bank contains only one word
        if word_bank.len() == 1 {
            println!("FOUND: {}", word_bank[0].data);
            break;
        }
        // Check if word bank is empty
        else if word_bank.len() == 0 {
            println!("Couldn't find a word!");
            break;
        }
        // If there are only a few words left, maybe the user will want to choose a different word
        else if word_bank.len() < 20 {
            for word in &word_bank {
                println!(
                    "Possible word: {} with cost {}",
                    word.data,
                    compute_query_cost(&word, &word_bank).0
                )
            }
        }

        guesses += 1;
    }
}
