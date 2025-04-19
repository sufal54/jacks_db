use jacs_db::JacsDb;
fn main() {
    let db = JacsDb::new("data".to_string());
    db.create_one("Hello World!".to_string());
    let data = db.read_all();
    for d in data {
        println!("{}", d);
    }
}
