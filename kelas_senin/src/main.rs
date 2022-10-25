// case
//
// 1. bikin fungsi untuk menampilkan warna dari setiap pakaian (kaos, celana, sepatu)
// 2. harus menampilkan setidaknya satu warna dari setiap pakaian

// pseudocode
//
// fungsi yang bernama setiap pakaian yang menerima parameter warna
// bikin variant warna yang tersedia
#[derive(Debug)]
enum Color {
    Hitam,
    Putih,
    Biru,
    Merah,
    Custom(String)
}

#[derive(Debug)]
struct WarnaKaos(Color);
impl WarnaKaos {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

#[derive(Debug)]
struct WarnaCelana(Color);
impl WarnaCelana {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

#[derive(Debug)]
struct WarnaSepatu(Color);
impl WarnaSepatu {
    fn new(color: Color) -> Self {
        Self(color)
    }
}


fn cetak_kaos (warna: WarnaKaos) {
    println!("Warna kaos ini adalah {:?}", warna);
}

fn cetak_celana (warna: WarnaCelana) {
    println!("Warna celana ini adalah {:?}", warna);
}

fn cetak_sepatu (warna: WarnaSepatu) {
    println!("Warna sepatu ini adalah {:?}", warna);
}

fn main() {
    let warna_kaos = WarnaKaos::new(Color::Biru);
    let warna_celana = WarnaCelana::new(Color::Merah);
    let warna_sepatu = WarnaSepatu::new(Color::Hitam);

    cetak_kaos(warna_kaos);
    cetak_celana(warna_celana);
    cetak_sepatu(warna_sepatu);
}
