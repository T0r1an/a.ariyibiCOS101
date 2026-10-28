struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    fn calculate_cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    let hp = Laptop {
        brand: String::from("HP"),
        price: 650000,
    };
    
    let ibm = Laptop {
        brand: String::from("IBM"),
        price: 755000,
    };
    
    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        price: 550000,
    };
    
    let dell = Laptop {
        brand: String::from("Dell"),
        price: 850000,
    };

    let hp_total = hp.calculate_cost(3);
    let ibm_total = ibm.calculate_cost(3);
    let toshiba_total = toshiba.calculate_cost(3);
    let dell_total = dell.calculate_cost(3);

    let grand_total = hp_total + ibm_total + toshiba_total + dell_total;

    println!("The grand total for 3 laptops of each brand is: N{}", grand_total);
}