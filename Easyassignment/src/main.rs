use std::fs::File;
use std::io::prelude::*;

struct Config {
    name: String,
    user_id: String,
    port: u16,
}

impl Config {
    fn from_file(path: &str) -> Config {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let name = lines.next().unwrap().to_string();
        let user_id = lines.next().unwrap().to_string();
        let port = lines.next().unwrap().parse().unwrap();

        Config { name, user_id, port }
    }
}

fn reading_from_file() {
    let config = Config::from_file("config.txt");
    println!("Full Name: {}", config.name);
    println!("ID #: {}", config.user_id);
    println!("port: {}", config.port);
}
fn main() {
    reading_from_file();
}
