use std::io::stdin;

fn what_is_your_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line");
    name.trim().to_lowercase()
}

fn main() {
    let visitor_list = ["bert", "steve", "fred"];
    let mut allow_them_in = false;
    println!("Hello, what's your name?");
    let your_name = what_is_your_name();
    for visitor in &visitor_list {
        if visitor == &your_name {
            allow_them_in = true;
        }
    }

    if allow_them_in {
        println!("Welcome, {}!", your_name)
    } else {
        println!("Sorry, {}, you're not on the list.", your_name)
    }
    // println!("Hello, {}", name);
    // println!("{:?}", name);
}
