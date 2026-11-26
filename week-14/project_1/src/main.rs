use std::io;
use std::io::Read;

fn administrator() {
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn project_manager() {
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn employee() {
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn customer() {
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn vendor() {
    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn main() {
    println!("Enter your role (Administrator, Project manager, Employee, Customer, Vendor):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let role = input.trim().to_lowercase();

    if role == "administrator" {
        administrator();
    } else if role == "project manager" {
        project_manager();
    } else if role == "employee" {
        employee();
    } else if role == "customer" {
        customer();
    } else if role == "vendor" {
        vendor();
    } else {
        println!("Invalid role entered.");
    }
}