use chrono::prelude::*;

fn main() {
    let local: DateTime<Local> = Local::now();
    println!("{}", local.format("%Y-%m-%d %H:%M:%S").to_string());
}
