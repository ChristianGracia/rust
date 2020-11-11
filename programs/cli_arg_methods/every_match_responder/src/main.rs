
// use arguments in run command annd this will say hi to any animal whos letter starts with c
//run with cargo run -- dog cat horse lizard

use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() > 0 {
        match_responder(args, 'c');
    }



}

fn match_responder(args: Vec<String>, letter: char) -> () {
    for a in args {
        if a.chars().next().unwrap() == letter {
            println!("Match found: {}", a);
        }
    }
}
