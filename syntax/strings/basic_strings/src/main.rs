fn main() {

    show_use_of_pointers();
    show_bytes();
    show_char_indices();
    show_enumerater();

    let random_string = String::from("lloooooo");

    //use of pointer allows more flexible methods, no need to copy over string
    println!("number of l's in pointer {}", count_l(&random_string));

}

fn show_enumerater(){

    let random_string = String::from("hi whats up");

    for (i, c) in random_string.chars().enumerate() {
        println!("{} = {}", i, c);
    }

}

fn show_char_indices(){

    let random_string = String::from("hi whats up");

    for (i, c) in random_string.char_indices() {
        println!("from char_indices char: {}, index: {}", i, c);
    }

}

fn show_use_of_pointers() {

    let pointer_string  = "pointer, this last for life of program, has no string methods";
    let pointer_string_two = "hi";

    println!("{} has chars and a length of {}", pointer_string_two, pointer_string_two.len());

}

fn show_bytes(){
    
    let byte_per_char_string = String::from("hi");
    let more_then_one_byte_string  = String::from("hi平");

    println!("{} has three characters but a length of {}", more_then_one_byte_string, more_then_one_byte_string.len());

    for c in byte_per_char_string.chars() {
        println!("char: {}", c);
    }

    for c in byte_per_char_string.bytes() {
        println!("{} bytes in this char", c);
    }

    for c in more_then_one_byte_string.chars() {
        println!("char: {}", c);
    }

    for c in more_then_one_byte_string.bytes() {
        println!("{} bytes in this char", c);
    }

}

fn count_l(s:&str) -> i32 {

    let mut res = 0;

    for c in s.chars() {
        if c == 'l' {
            res += 1;
        }
    }

    res

}
