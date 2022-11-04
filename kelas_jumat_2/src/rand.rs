extern crate rand;

fn main() {
    let random_num1 = rand::random::<i32>();
    println!("random num1: {}", random_num1);

    let random_num2 = rand::random::<i32>();
    println!("random num2: {}", random_num2);

    let random_char = rand::random::<char>();
    println!("random char: {}", random_char);

    use rand::Rng;
    let mut rng = rand::thread_rng();
    if rng.gen() {
        println!("This message has 50-50 chance to print");
    }

    let random_num3 = rng.gen_range(0..10);
    println!("random num3: {}", random_num3);

    let random_float = rng.gen_range(0.0..1.0);
    println!("random float: {}", random_float);
}