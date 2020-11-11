use std::collections::HashMap;
use std::env::args;

fn main() {


    let mut random_var = HashMap::new();

    random_var.insert(3, "cat");
    random_var.insert(5, "dog");

    let r = random_var.get(&3).unwrap_or(&"No string");

    //using unwrap_or
                                        // & here because get always returns a pointer
    let c = random_var.get(&8).unwrap_or(&"No string");

    println!("{}", r);

    println!("{}", c);

    // testing on results of parse
    match "3".parse::<f32>(){
        Ok(v)=> println!("ok {}", v),
        Err(e) => println!("Error: {}", e)
    }

    match "e".parse::<f32>(){
        Ok(v)=> println!("ok {}", v),
        Err(e) => println!("Error: {}", e)
    }


    // this would return truthy with cargo run -- a i d using extra args
    match get_arg(3){
         
        Ok(v)=> println!("ok {}", v),
        Err(e) => println!("Error: {}", e)
    }

}

fn get_arg(n:usize) -> Result<String, String> {
    for (i, a) in args().enumerate(){
        if i == n {
            return Ok(a)
        }

        println!("i = {}", i);
    }
    Err("Not enough Args".to_string())
}