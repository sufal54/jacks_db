// use jacs_db::{ JacsDb, json };
use json_driver::Serialize;

#[derive(Serialize)]
struct Person {
    name: String,
    age: u8,
    sub: Vec<char>,
}
#[derive(Serialize)]
struct Person2 {
    id: u16,
    person: Person,
}
fn main() {
    let a = Person {
        name: "mona".to_string(),
        age: 20,
        sub: vec!['a', 'b'],
    };

    println!("Person1: {}", a.serialize());
    let b = Person2 {
        id: 154,
        person: a,
    };
    println!("Person2: {}", b.serialize());
}
