fn main() {
   loopMethod(10);
}

fn loopMethod(limit: i32){

    let mut counter = 0;
    loop {
        counter += 1;
        println!("{}", counter);
        if counter >= limit {
            return;
        }
    }
}