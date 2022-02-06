// #![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unreachable_code)]
// #![allow(unused_assignments)]

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fmt;
use std::collections::HashMap;

fn get_word_bank(fname: &str) -> Vec<String> {
    let path = Path::new(fname);
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => panic!("Could not open {display} because {err}", display=path.display(), err=err)
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(err) => panic!("Could not read {display} because {err}", display=path.display(), err=err)
    };

    let word_bank: Vec<&str> = contents.split('\n').collect();
    let mut word_bank_string: Vec<String> = Vec::new();
    for word in word_bank {
        word_bank_string.push(word.to_string());
    }
    word_bank_string
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Color {
    GRAY,
    YELLOW,
    GREEN
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::GRAY => write!(f, "{}", "\u{2B1B}"),
            Color::YELLOW => write!(f, "{}", '\u{1F7E8}'),
            Color::GREEN => write!(f, "{}", '\u{1F7E9}')
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Filter {
    colors: [Color; 5]
}
impl Filter {
    fn new() -> Self {
        Filter {
            colors: [Color::GRAY, Color::GRAY, Color::GRAY, Color::GRAY, Color::GRAY]
        }
    }

}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}{}{}", self.colors[0], self.colors[1], self.colors[2], self.colors[3], self.colors[4])
    }
}

fn compute_filter(query: &String, secret: &String) -> Filter {
    let mut filter = Filter::new();

    for i in 0..query.len() {
        // Skip grays
        if !secret.as_bytes().contains(&(query.as_bytes()[i])) {
            continue;
        }
        // At this point, filters.colors[i] is either yellow or green
        // Green
        else if query.as_bytes()[i] == secret.as_bytes()[i] {
            filter.colors[i] = Color::GREEN;
        }
        else {
            filter.colors[i] = Color::YELLOW;
        }
    }

    filter
}

fn compute_filters_to_secret_candidates_for_query(query: &String, secret_candidates: &Vec<String>) -> HashMap<Filter, Vec<String>> {
    let mut filters_to_secret_candidates: HashMap<Filter, Vec<String>> = HashMap::new();
    for secret in secret_candidates {
        let filter = compute_filter(&query, &secret);
        filters_to_secret_candidates.entry(filter).or_insert(Vec::new());
        filters_to_secret_candidates.get_mut(&filter).unwrap().push(secret.to_string())
    }
    filters_to_secret_candidates
}

fn compute_hashmap_cost(filters_to_secret_candidates_for_query: &HashMap<Filter, Vec<String>>) -> usize {
    let mut max_count: usize = 0;

    for (_, secret_candidates_from_filter) in filters_to_secret_candidates_for_query.iter() {
        // cost is worst-case performance
        if secret_candidates_from_filter.len() > max_count {
            max_count = secret_candidates_from_filter.len();
        }
    }

    max_count
}

fn compute_query_cost(query: &String, word_bank: &Vec<String>) -> (usize, HashMap<Filter, Vec<String>>) {
    let filters_to_secret_candidates_for_query = compute_filters_to_secret_candidates_for_query(&query, &word_bank);
    let hashmap_cost = compute_hashmap_cost(&filters_to_secret_candidates_for_query);
    (hashmap_cost, filters_to_secret_candidates_for_query)
}

/*
* TODO: Check that input is exactly five words
* TODO: CHeck that all words are either gray, yellow, or green
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
            _ => ()
        }
    }

    filter
}

fn compute_best_query(word_bank: &Vec<String>) -> (String, HashMap<Filter, Vec<String>>) {
    let mut minimum_cost_query: String = word_bank[0].clone();
    let mut minimum_cost: usize = usize::MAX;
    let mut minimum_cost_filters_to_secret_candidates: HashMap<Filter, Vec<String>> = HashMap::new();

    for query in word_bank {
        let (cost, filters_to_secret_candidates) = compute_query_cost(query, word_bank);
        if cost < minimum_cost {
            minimum_cost = cost;
            minimum_cost_query = query.clone();
            minimum_cost_filters_to_secret_candidates = filters_to_secret_candidates;
        }
    }
    (minimum_cost_query, minimum_cost_filters_to_secret_candidates)
}

fn main() {
    // Uncomment to test what filter will be generated by a query and a secret
    println!("{}", compute_filter(&String::from("sands"), &String::from("skill")));
    return;

    // Main application
    let mut word_bank = get_word_bank("sgb-words.txt");
    let mut best_query: String;
    let mut best_query_filters_to_secret_candidates: HashMap<Filter, Vec<String>>;
    let mut filter: Filter;
    loop {
        // Get the best guess
        let (a,  b) = compute_best_query(&word_bank);
        best_query = a;
        best_query_filters_to_secret_candidates = b;

        // Supply best guess to user
        println!("Best guess: {} ({})", best_query, compute_query_cost(&best_query, &word_bank).0);

        // filter = get_filter_from_input(); // Get filter from user
        filter = compute_filter(&best_query, &String::from("skill")); // Calculate filter automatically when secret word is known (testing only)
        println!("Filter received: {}", filter);

        // word_bank is the list of words that the filter maps to
        word_bank = best_query_filters_to_secret_candidates.get(&filter).unwrap().clone();

        // Check if word bank contains only one word
        if word_bank.len() == 1 {
            println!("FOUND: {}", word_bank[0]);
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
                println!("Possible word: {} with cost {}", word, compute_query_cost(&word, &word_bank).0)
            }
        }
    }
}

