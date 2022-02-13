use std::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub const WORD_LENGTH: usize = 5;
const QUERY_FILENAME: &str = "queries.txt";
const SOLUTION_FILENAME: &str = "solutions.txt";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Word {
    pub data: String,
    map: HashMap<u8, HashSet<usize>>,
}

impl Word {
    pub fn new(word_string: &str) -> Self {
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

pub fn get_query_bank() -> Vec<Word> {
    get_word_bank(QUERY_FILENAME)
}

pub fn get_solution_bank() -> Vec<Word> {
    get_word_bank(SOLUTION_FILENAME)
}

// TODO: Return Option in case file name does not exist
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
pub enum Color {
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

#[derive(PartialEq, Eq, Hash, Clone, Debug, Copy)]
pub struct Filter {
    pub colors: [Color; WORD_LENGTH],
}

impl Filter {
    fn new() -> Self {
        Filter {
            colors: [Color::GRAY, Color::GRAY, Color::GRAY, Color::GRAY, Color::GRAY],
        }
    }

    pub fn is_green(&self) -> bool {
        self.colors == [Color::GREEN; 5]
    }

    fn to_value(&self) -> u8 {
        let mut value: u8 = 0;
        for (i, color) in self.colors.iter().enumerate() {
            value += 3u8.pow((self.colors.len() as u32) - 1 - i as u32) * match color {
                Color::GRAY => 0,
                Color::YELLOW => 1,
                Color::GREEN => 2, 
            };
        }
        
        value
    }
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}{}{}", self.colors[0], self.colors[1], self.colors[2], self.colors[3], self.colors[4])
    }
}

pub fn compute_filter(query: &Word, secret: &Word) -> Filter {
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

pub fn get_filter_from_input() -> Filter {
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

pub fn compute_best_query(word_bank: &Vec<Word>, secret_candidates: &Vec<Word>) -> Word {
    compute_best_query_and_hashmap(word_bank, secret_candidates).0
}

fn compute_best_query_and_hashmap(word_bank: &Vec<Word>, secret_candidates: &Vec<Word>) -> (Word, HashMap<Filter, Vec<Word>>) {
    let mut minimum_cost_query: Word = word_bank[0].clone();
    let mut minimum_cost: u32 = u32::MAX;
    let mut minimum_cost_hashmap: HashMap<Filter, Vec<Word>> = HashMap::new();

    for query in word_bank {
        let (cost, hashmap) = compute_query_cost(query, secret_candidates);
        if cost < minimum_cost {
            minimum_cost_query = query.clone();
            minimum_cost = cost;
            minimum_cost_hashmap = hashmap;

        }
    }
    (minimum_cost_query, minimum_cost_hashmap)
}

// TODO: Return Option and check in main, in case user-provided filter does not exist in hashmap
pub fn filter_secret_candidates(query: &Word, filter: &Filter, secret_candidates: &Vec<Word>) -> Vec<Word> {
    let hashmap = compute_query_cost(&query, &secret_candidates).1;
    return hashmap.get(&filter).unwrap().to_vec();
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
                Some(_) => (), // Will never reach here because we've already checked that key does not exist 
                None => () // Don't need the old list of secret candidates that the filter maps to 
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

pub fn play(arg_known_secret: &str) {
    let word_bank = get_query_bank();
    let mut secret_candidates = get_solution_bank();

    let mut best_query = Word::new("aesir");

    let mut guesses = 1;
    let mut filter: Filter;

    loop {
        // If a guess has already been made, need to compute the next best guess
        if guesses > 1 {
            best_query = compute_best_query(&word_bank, &secret_candidates);
        }

        // Supply best guess to user
        println!("Best guess: {}", best_query.data);

        // If secret word is not known, get filter from user
        if arg_known_secret.len() == 0 {
            filter = get_filter_from_input();
        }
        // Calculate filter automatically when secret word is known (testing only)
        else {
            filter = compute_filter(&best_query, &Word::new(&arg_known_secret));
        }

        // If filter is all green, print solution and return
        if filter.is_green() {
            println!("FOUND: {} in {} guess{}", best_query.data, guesses, if guesses != 1 {"es"} else {""});
            break;
        }

        // secret_candidates is the list of possible solutions that the filter maps to
        secret_candidates = filter_secret_candidates(&best_query, &filter, &secret_candidates);

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

#[derive(Debug)]
enum StringFilter {
    String(String),
    Filter(Filter),
}

impl fmt::Display for StringFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StringFilter::String(p) => write!(f, "{}", p),
            StringFilter::Filter(p) => write!(f, "{}", p),
        }
    }
}

fn get_sorted_filters(hashmap: &HashMap<Filter, Vec<Word>>) -> Vec<Filter> {
    let mut filters: Vec<Filter> = Vec::new();
    for filter in hashmap.keys() {
        filters.push(filter.clone());
    }
    filters.sort_by(|a, b| a.to_value().cmp(&b.to_value()));
    
    filters
}

fn dfs(word_bank: &Vec<Word>, filter: Filter, secret_candidates: Vec<Word>, root_to_leaf_path: &mut Vec<StringFilter>, file: &mut File) {
    root_to_leaf_path.push(StringFilter::Filter(filter));
    if secret_candidates.len() == 1 {
        root_to_leaf_path.push(StringFilter::String(secret_candidates[0].data.clone()));
        // Do whatever needs to be done with root_to_leaf_path
        for (idx, element) in root_to_leaf_path.iter().enumerate() {
            // print!("{} ", element);
            if idx + 1 < root_to_leaf_path.len() {
                write!(file, "{} ", element).expect("Couldn't write to file.")
            }
            else {
                write!(file, "{}\n", element).expect("Couldn't write to file.")
            }
        }
    }
    else {
        let (query, hashmap) = compute_best_query_and_hashmap(&word_bank, &secret_candidates);
        root_to_leaf_path.push(StringFilter::String(query.data));

        let filters = get_sorted_filters(&hashmap);
        
        for filter_ref in filters.iter() { 
            let filter = (*filter_ref).clone();
            dfs(&word_bank, filter, hashmap.get(&filter).expect("Couldn't find the filter in the hash table.").to_owned(), root_to_leaf_path, file); 
        }
    }

    root_to_leaf_path.pop();
    root_to_leaf_path.pop();
}


pub fn solve() {
    let word_bank = get_query_bank();
    let secret_candidates = get_solution_bank();
    let best_query = Word::new("aesir");
    let hashmap = compute_filters_to_secret_candidates_for_query(&best_query, &secret_candidates);

    let filters = get_sorted_filters(&hashmap);

    let mut root_to_leaf_path: Vec<StringFilter> = vec![StringFilter::String(best_query.data)];
    let mut file = File::create("solution_map.txt").unwrap();
    for filter_ref in filters.iter() { 
        let filter = (*filter_ref).clone();
        dfs(&word_bank, filter, hashmap.get(&filter).unwrap().to_owned(), &mut root_to_leaf_path, &mut file); 
    }
}