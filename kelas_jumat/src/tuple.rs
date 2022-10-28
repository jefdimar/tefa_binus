#[derive(Debug)]
struct TestTuple {
    tuple_1: (String, String, String),
    tuple_2: (String, String, String),
}

fn main() {
    let tuple_1 = (String::from("henry"), String::from("raymond"), String::from("richie"));
    let tuple_2 = (String::from("farell"), String::from("william"), String::from("eric"));
    let mahasiswa = TestTuple {
        tuple_1,
        tuple_2
    };
    let print_mhsw = mahasiswa.tuple_1.2;
    println!("{:?}", print_mhsw);
}