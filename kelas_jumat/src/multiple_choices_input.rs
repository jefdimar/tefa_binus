// Tentuin multiple choices
#[derive(Debug)]
enum Currency {
    Rupiah,
    Dollar,
    Rupee,
    Yen
}
// implementasi di enum fn untuk mengconvert dari type data input ke enum
impl Currency {
    fn convert_str(e: String) -> Result<Currency, String> {
        match e {
            String::from("Rupiah") => Ok(Currency::Rupiah),
            String::from("Dollar") => Ok(Currency::Dollar),
            String::from("Rupee") => Ok(Currency::Rupee),
            String::from("Yen") => Ok(Currency::Yen),
            _ => Err(String::from("tidak terdaftar"))
        }
    }
}

fn main() {
    // Dipanggil sebelum proses output
    println!("{:?}", Currency::convert_str("Rupiah".to_string()).unwrap());
}
