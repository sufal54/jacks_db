// use jacs_db::{ JacsDb, json };
use json_driver::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    sub: Person2,
}

#[derive(Serialize, Deserialize)]
struct Person2 {
    name: String,
    roll: u8,
}
fn main() {
    let input =
        r#"{
        "name": "sulekha  bala","age": 60,"sub": {"name":"yo","roll":10 }
    }"#;

    let a: Person = input.parse().unwrap();
    let b = a.serialize();
    println!("b String: {}", b);
    println!("{:?}", a);
    let b: Person = b.as_str().parse::<Person>().unwrap();
    println!("b struct: {:?}", b);
}
