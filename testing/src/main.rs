use std::io::{self, Read, Write};

struct Person {
    name: String,
    age: u32,
}

fn main() {
    let mut buffer = String::new();

    print!("What's your name? ");
    io::stdout().flush().unwrap(); //empties all characters in the terminal
    io::stdin().read_line(&mut buffer).unwrap(); //read console from output
    let name = buffer.trim().to_string(); //makes variable name equal the user input
    buffer.clear();

    print!("How old are you? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let age = buffer.trim().parse().unwrap();

    let person = Person { name, age };
    println!("Hi {}, you are {} years old!", person.name, person.age);
}