// Shape Calculator - CSC 101 Week 7 Project
// One function per shape. Reads user input, prints area or volume.

use std::f64::consts::PI;
use std::io;

// Reads a number from the user with a prompt.
fn read_number(prompt: &str) -> f64 {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        match input.trim().parse::<f64>() {
            Ok(n) => return n,
            Err(_) => println!("Not a valid number. Try again.\n"),
        }
    }
}

// --- Shape functions ---

fn trapezium_area(height: f64, base1: f64, base2: f64) -> f64 {
    height / 2.0 * (base1 + base2)
}

fn rhombus_area(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

fn parallelogram_area(base: f64, altitude: f64) -> f64 {
    base * altitude
}

fn cube_surface_area(side: f64) -> f64 {
    6.0 * side * side
}

fn cylinder_volume(radius: f64, height: f64) -> f64 {
    PI * radius * radius * height
}

fn main() {
    println!("=================================================");
    println!("            THE SHAPE CALCULATOR");
    println!("        CSC 101 Practical - Week 7 Project");
    println!("=================================================");
    println!("Your MTH 101 lecturer sends regards.\n");

    loop {
        println!("Which shape would you like to work with?");
        println!("  1. Trapezium      (area)");
        println!("  2. Rhombus        (area)");
        println!("  3. Parallelogram  (area)");
        println!("  4. Cube           (surface area)");
        println!("  5. Cylinder       (volume)");
        println!("  0. Quit");
        println!();

        let choice = read_number("Enter your choice:") as i32;

        match choice {
            1 => {
                println!("\n--- Trapezium (area) ---");
                let height = read_number("Enter the height:");
                let base1 = read_number("Enter the first base:");
                let base2 = read_number("Enter the second base:");
                let area = trapezium_area(height, base1, base2);
                println!("The area of the trapezium is {:.2}\n", area);
            }
            2 => {
                println!("\n--- Rhombus (area) ---");
                let d1 = read_number("Enter the first diagonal:");
                let d2 = read_number("Enter the second diagonal:");
                let area = rhombus_area(d1, d2);
                println!("The area of the rhombus is {:.2}\n", area);
            }
            3 => {
                println!("\n--- Parallelogram (area) ---");
                let base = read_number("Enter the base:");
                let altitude = read_number("Enter the altitude:");
                let area = parallelogram_area(base, altitude);
                println!("The area of the parallelogram is {:.2}\n", area);
            }
            4 => {
                println!("\n--- Cube (surface area) ---");
                let side = read_number("Enter the length of one side:");
                let area = cube_surface_area(side);
                println!("The surface area of the cube is {:.2}\n", area);
            }
            5 => {
                println!("\n--- Cylinder (volume) ---");
                let radius = read_number("Enter the radius:");
                let height = read_number("Enter the height:");
                let volume = cylinder_volume(radius, height);
                println!("The volume of the cylinder is {:.2}\n", volume);
            }
            0 => {
                println!("\nThank you for using the Shape Calculator. Goodbye!");
                break;
            }
            _ => println!("'{}' is not one of the options. Try 0 to 5.\n", choice),
        }
    }
}