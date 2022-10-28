#[derive(Debug)]
struct ShippingBox {
    depth: i32,
    height: i32,
    width: i32,
    name: String,
    status: bool,
    price: f64,
    contain: Vec<String>,
    item: Item,
    status_reject: RejectStatus
}

#[derive(Debug)]
enum RejectStatus {
    Reject,
    Valid,
    Unknown
}

#[derive(Debug)]
struct Item {
    name: String,
    type_item: String,
}

fn main() {
    let mut contain_item = Vec::new();
    contain_item.push("sepatu".to_string());
    contain_item.push("parfum".to_string());
    contain_item.push("konsol".to_string());
    let result = ShippingBox {
        depth: 0,
        height: 1,
        width: 2,
        name: String::from("jefri"),
        status: false,
        price: 50000.0,
        contain: contain_item,
        item: Item {
            name: String::from("sepatu"),
            type_item: String::from("fashion")
        },
        status_reject: RejectStatus::Reject
    };

    let print_box_name = result.item;
    println!("{:?}", print_box_name)
}