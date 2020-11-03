fn main() {
   loop_method(10);
   for_loop(10);
   array_loop();
}

fn loop_method(limit: i8){

    let mut counter = 0;
    loop {
        counter += 1;
        println!("{} in loop", counter);
        if counter >= limit {
            return;
        }
    }
}

fn for_loop(limit: i8){

    for n in 0..limit {
          println!("{} in for loop", n);
    }

}

fn array_loop(){

    let mut v = Vec::new();
    let c = vec![3,4,5,6,7,87,8];

    v.push(4);
    v.push(5);
    v.push(6);

    let mut counter: i8 = 0;

    'outer: for i in 1..10 {
        for n in &c {
            counter += 1;
        
            if counter == 5 {
                println!("breaking with label on {}", i);
                break 'outer;
            }
            println!("{} in array", n);
        }
    }

    for n in c {
        if n % 2 == 0 {
            continue;
        }
        println!("{} in other array", n);
    }
}