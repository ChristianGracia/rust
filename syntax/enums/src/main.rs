
#[derive(Debug)]
pub struct User {
    size: i32,
    count: u32,
    name: String,
}


#[derive(Debug)]
pub enum Games {
    GameId(i32), GameNumber(i32), GameUser(User)
}


fn main() {
    let t = Games::GameId(3);
    println!("id {:?}", t);

    let c = Games::GameUser(User{size: 2, count: 3, name: "cg".to_string()});
 

    match t {
        Games::GameId(n) => println!("This game has id {}", n),
        d => println!("{:?}", d),
    }

}
