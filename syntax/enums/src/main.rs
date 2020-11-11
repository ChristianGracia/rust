
#[derive(Debug)]
pub struct User {
    size: i32,
    count: u32,
    name: String,
}


#[derive(Debug)]
pub enum Games {
    GameId(i32), GameNumber(i32), GameUser(User), GameMessage(i32, String)
}


fn main() {
    use self::Games::*;
    // allows not using Games:: ex. 
    //  let c = Games::GameUser(User{size: 2, count: 3, name: "cg".to_string()});

    let t = GameId(3);

    let d =  GameMessage(2, "dog".to_string());

    println!("id {:?}", t);

    let c = GameUser(User{size: 2, count: 3, name: "cg".to_string()});

    if let GameId(n) = t {
        println!("n = {}", n);
    }

    if let GameMessage(n, s) = d {
        println!("n = {}, s = {}", n, s);
    }


}
