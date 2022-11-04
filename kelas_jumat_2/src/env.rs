use std::env;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    // println!("got following parameter : ");
    // for arg in env::args() {
    //     println!("- {}", arg);
    // }

    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
}