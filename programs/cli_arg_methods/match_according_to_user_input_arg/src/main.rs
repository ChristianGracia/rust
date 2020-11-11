// use wanted regex to match in first argument and items to be matches in the args after
//run with cargo run -- dog cat horse lizard dog
// this would match for dog accross cat horse lizard dog

use std::env;

fn main() {

    let mut args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let user_input_regex : String = args.remove(1);
        println!("match regex: {}", user_input_regex);
        match_responder(args,  user_input_regex.to_string());
    }
    else {
        println!("no args");
    }
}

fn match_responder(args: Vec<String>, user_input_regex: String) -> () {
    for a in args {
        if a == user_input_regex {
            println!("match found: {}", a);
        }
    }
}