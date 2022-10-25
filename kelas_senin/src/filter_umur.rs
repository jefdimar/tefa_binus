// case
//
// bikin fungsi / program yang menampilkan warna favorit dari seseorang
// dan saring(filter) untuk menampilkan hanya yang berumur dibawah 22 tahun

// Standart struct
struct Orang {
    name: String,
    fav_color: String,
    age: i32
}

// Helper func to print output to terminal
fn print(data: &str) {
    println!("{:?}", data);
}

fn main () {

    // Bundle of data for process
    let data = vec![
        Orang {
            name: String::from("Henry"),
            fav_color: String::from("Biru"),
            age: 21
        },
        Orang {
            name: String::from("Raymond"),
            fav_color: String::from("Merah"),
            age: 21
        },
        Orang {
            name: String::from("Farrel"),
            fav_color: String::from("Tosca"),
            age: 22
        }
    ];
    
    // looping di rust
    // 'for' - alias element dari data yang sedang di proses - 'in' - data yang mau di loop
    for orang in data {
        if orang.age < 22 {
            print(&orang.name);
            print(&orang.fav_color);
        }
    }
}