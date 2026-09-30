use std::io;

fn main() {
    // Display the menu
    println!("Restaurant Menu");
    println!("P - Poundo Yam / Edinkaiko Soup - N3200");
    println!("F - Fried Rice & Chicken - N3000");
    println!("A - Amala & Ewedu Soup - N2500");
    println!("E - Eba & Egusi Soup - N2000");
    println!("W - White Rice & Stew - N2500");

    // Read the food letter
    println!("Enter food letter (P, F, A, E, W):");
    let mut food_input = String::new();
    io::stdin()
        .read_line(&mut food_input)
        .expect("Failed to read input");
    let food = food_input.trim();

    // Read the quantity
    println!("Enter quantity:");
    let mut quantity_input = String::new();
    io::stdin()
        .read_line(&mut quantity_input)
        .expect("Failed to read input");
    let quantity: f32 = quantity_input
        .trim()
        .parse()
        .expect("Quantity must be a number");

    // Decide the price per unit
    let price: f32;
    if food == "P" {
        price = 3200.0;
    } else if food == "F" {
        price = 3000.0;
    } else if food == "A" {
        price = 2500.0;
    } else if food == "E" {
        price = 2000.0;
    } else if food == "W" {
        price = 2500.0;
    } else {
        println!("Invalid food letter. Please run the program again.");
        return;
    }

    // Compute the total
    let mut total = price * quantity;

    // Apply 5% discount if total is greater than N10,000
    if total > 10000.0 {
        let discount = total * 0.05;
        total = total - discount;
        println!("Discount applied: N{:.2}", discount);
    }

    // Show the final total
    println!("Total charge: N{:.2}", total);
}