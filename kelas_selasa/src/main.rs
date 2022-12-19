use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
struct Bill {
    name: String,
    amount: f64
}

struct Bills {
    inner: HashMap<String, Bill>
}
impl Bills {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.insert(bill.name.clone(), bill);
    }

    fn get_all(&self) -> Vec<&Bill> {
        let mut bills = vec![];
        for bill in self.inner.values() {
            bills.push(bill)
        }
        bills
    }

    fn remove(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
    }

    fn update(&mut self, name: &str, amount: f64) -> bool {
        match self.inner.get_mut(name) {
            Some(bill) => {
                bill.amount = amount;
                true
            },
            None => false
        }
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again")
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

fn get_bill_amount() -> Option<f64> {
    print!("Amount:");
    loop {
        let input = match get_input() {
            Some(input) => input,
            None => return None
        };
        if &input == "" {
            return None;
        }
        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please enter a number")
        }
    }
}

fn add_bill_menu(bills: &mut Bills) {
    println!("Bill name: ");
    let name = match get_input() {
        Some(input) => input,
        None => return
    };
    let amount  = match get_bill_amount() {
        Some(amount) => amount,
        None => return
    };
    let bill = Bill { name, amount };
    bills.add(bill);
    println!("Bill added");
}

fn remove_bill_menu(bills: &mut Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
    println!("Enter bill name to remove:");
    let name = match get_input() {
        Some(input) => input,
        None => return
    };
    if bills.remove(&name) {
        println!("Removed")
    } else {
        println!("Bill not found")
    }
}

fn update_bill_menu(bills: &mut Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
    println!("Enter bill to update:");
    let name = match get_input() {
        Some(input) => input,
        None => return
    };
    let amount  = match get_bill_amount() {
        Some(amount) => amount,
        None => return
    };
    if bills.update(&name, amount) {
        println!("Updated")
    } else {
        println!("Bill not found")
    }
}

fn view_bills_menu(bills: &Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
}

fn main_menu() {
    fn show() {
        println!("");
        println!("=== Manage Bills ===");
        println!("1. Add Bills");
        println!("2. View Bills");
        println!("3. Remove Bills");
        println!("4. Update Bills");
        println!("");
        println!("Enter Selection");
    }

    let mut bills = Bills::new();

    loop {
        show();
        let input = match get_input() {
            Some(input) => input,
            None => return
        };
        match input.as_str() {
            "1" => add_bill_menu(&mut bills),
            "2" => view_bills_menu(&bills),
            "3" => remove_bill_menu(&mut bills),
            "4" => update_bill_menu(&mut bills),
            _ => break
        }
    }
}
fn main() {
    main_menu()
}

