fn print_it(str: &str) {
    println!("{:?}", str);
}

fn main() {
    let input = String::from("testing");

    print_it(&input);

    println!("{:?}", input);
}