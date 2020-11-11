
// use arguments in run command annd this will say hi to any animal whos letter starts with c
//run with cargo run -- dog cat horse lizard

use std::env::args;

fn main() {

    match first_letter_checker('c') {
        Ok(n) => println!("{}", n),
        Err(e) => println!("{}", e),
    }

}

fn first_letter_checker(n:char) -> Result<String, String> {
    for (i , a) in args().enumerate(){
        if a.chars().next().unwrap() == n {
            return Ok(a)
        }
    }
    Err("No".to_string())
}
