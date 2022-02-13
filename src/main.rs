mod wordle;
mod test;
extern crate argparse;

use argparse::{ArgumentParser, Store, StoreTrue};


fn main() {
    let mut arg_known_secret: String = "".to_string();
    let mut arg_generate_map: bool = false;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Solve Wordle.");
        ap.refer(&mut arg_known_secret).add_option(&["--secret"], Store, "Simulate program on a known secret.");
        ap.refer(&mut arg_generate_map).add_option(&["--generate-map"], StoreTrue, "Generate a text file mapping the behavior of the solver for all solutions.");
        ap.parse_args_or_exit();
    }

    if !arg_generate_map {
        wordle::play(&arg_known_secret);
    }
    else {
        wordle::solve();
    }

    // // TODO: Check that word_bank and secret_candidates exist by returning Results from get_query_bank and get_solution_bank
    // let word_bank = wordle::get_query_bank();
    // let mut secret_candidates = wordle::get_solution_bank();

    // // Compute best first guess if not supplied in command line args
    // let mut best_query = wordle::Word::new("aesir");

    // let mut guesses = 1;
    // let mut filter: wordle::Filter;

    // loop {
    //     // If a guess has already been made, need to compute the next best guess
    //     if guesses > 1 {
    //         best_query = wordle::compute_best_query(&word_bank, &secret_candidates);
    //     }

    //     // Supply best guess to user
    //     println!("Best guess: {}", best_query.data);

    //     // If secret word is not known, get filter from user
    //     if arg_known_secret.len() == 0 {
    //         filter = wordle::get_filter_from_input();
    //     }
    //     // Calculate filter automatically when secret word is known (testing only)
    //     else {
    //         filter = wordle::compute_filter(&best_query, &wordle::Word::new(&arg_known_secret));
    //     }

    //     // If filter is all green, print solution and return
    //     if filter.is_green() {
    //         println!("FOUND: {} in {} guess{}", best_query.data, guesses, if guesses != 1 {"es"} else {""});
    //         break;
    //     }

    //     // secret_candidates is the list of possible solutions that the filter maps to
    //     secret_candidates = wordle::filter_secret_candidates(&best_query, &filter, &secret_candidates);

    //     // Check if secret_candidates contains only one word
    //     match secret_candidates.len() {
    //         0 => {
    //             println!("Couldn't find a word!");
    //             break;
    //         },
    //         1 => {
    //             guesses += 1;
    //             println!("FOUND: {} in {} guess{}", &secret_candidates[0].data, guesses, if guesses > 1 { "es" } else { "" });
    //             break;
    //         },
    //         _ => {
    //             guesses += 1;
    //         },
    //     }
    // }
}
