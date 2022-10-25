// case
//
// fungsi untuk menghandle program error di suatu code

use thiserror::Error;

// Main variant
// Pengelompokan error dari sebuah program
#[derive(Debug, Error)]
enum ProgramError {
    // Error di menu/ item
    #[error("menu error")]
    Menu(#[from] MenuError),

    // Error di program math
    #[error("math error")]
    Math(#[from] MathError)
}

// Sub Varian dari program Error
// Variant Error di menu / item
#[derive(Debug, Error)]
enum MenuError {
    #[error("menu item not found")]
    NotFound
}

// Sub Varian dari program Error
// Variant Error di program math
#[derive(Debug, Error)]
enum MathError {
    #[error("divide by zero error")]
    DivideByZero
}

fn pembagian (angka: i32, pembagi: i32) -> Result<i32, MathError> {
    if pembagi == 0 {
        Err(MathError::DivideByZero)
    } else {
        let result = angka / pembagi;
    
        Ok(result)
    }
    
}
fn main() {
    let cetak_bagi = pembagian(2,0).unwrap();
    println!("{:?}", cetak_bagi);
}
