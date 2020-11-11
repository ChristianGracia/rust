
// use arguments in run command and this will find a match to any arg whos letter starts with c
//run with cargo run -- dog cat horse lizard

use std::env;

fn main() {
    match_responder(env::args().collect(), 'c');
    match_responder_better();
}

fn match_responder(args: Vec<String>, letter: char) -> () {
    for a in args {
        if a.chars().next().unwrap() == letter {
            println!("Match found: {}", a);
        }
    }
}

fn match_responder_better() -> () {
    for a in env::args() {
        if let Some(c) = a.chars().next() {
            match c {
                'c' | 'C' => println!("Match found in v2: {}", a),
                _ => {}
            }
        }
    }
}
