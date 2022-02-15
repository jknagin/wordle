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
}
