pub struct PubStruct {
    name: String,

}

#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    height: i32,

}

impl User {
    fn simple_string(&self) -> String {
        return format!("{} - {} - {}", self.name, self.age, self.height);
    }

    fn grow(&mut self, h:i32) {
        self.height += h;
    }

    fn die(self){
        println!("{} is dead", self.name);
    }
}

fn main() {
    let mut u = User{
        name: "c".to_string(),
        age: 33,
        height: 109
    };


    println!("user is {:?}", u);

    u.grow(100);

    println!("User is {}", u.simple_string());

    u.die();

    // this causes error because user is consumed by the function because of not using a pointer
    // u.die();

    
}
