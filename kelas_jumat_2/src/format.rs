use std::fmt::format;

fn main() {
    let colour = "red";
    let favourite = format!("my fav colour is {}", colour);
    println!("{}", favourite);

    let hello = "hello";
    let telkom = "telkom";
    let hello_telkom = format!("{} {}", hello, telkom);
    println!("{}", hello_telkom);

    let fav_num = format!("my fav number is {}", 42);
    println!("{}", fav_num);

    let duck_duck_goose = format!("{0}, {0}, {0}, {1}", "duck", "goose");
    println!("{}", duck_duck_goose);

    let introduction = format!("my surname is {surname}, {surname} {forname}", surname = "Jefri", forname = "Dimar");
    println!("{}", introduction);
}