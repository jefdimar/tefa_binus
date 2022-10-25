// case
//
// bikin suatu fungsi untuk mengkalikan 3 dari suatu koleksi data
// lalu hasil dari pengkalian tersebut disaring(filter) yang datanya lebih dari 100
// setelah itu tampilkan dengan metode for loop

fn main() {

    // Reference to https://doc.rust-lang.org/std/index.html
    let data: Vec<_> = vec![31, 32, 33, 34, 35, 36]
        .iter()
        .map(|element| element * 3)
        .filter(|e| e > &100)
        .collect();

    for num in data {
        println!("Hasil pemograman data {:?}", num);
    }
}