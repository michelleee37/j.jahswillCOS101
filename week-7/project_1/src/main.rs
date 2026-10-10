use std::f64::consts::PI;
use std::io::{self, Write};

// Prompt the user and read a positive number (re-asks on bad input).
fn read_number(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Could not read input, try again.");
            continue;
        }

        match input.trim().parse::<f64>() {
            Ok(n) if n >= 0.0 => return n,
            Ok(_) => println!("Please enter a non-negative number."),
            Err(_) => println!("That is not a valid number, try again."),
        }
    }
}

// Trapezium, area = height / 2 * (base1 + base2)
fn trapezium_area(height: f64, base1: f64, base2: f64) -> f64 {
    height / 2.0 * (base1 + base2)
}

// Rhombus, area = 1/2 * diagonal1 * diagonal2
fn rhombus_area(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

// Parallelogram, area = base * altitude
fn parallelogram_area(base: f64, altitude: f64) -> f64 {
    base * altitude
}

// Cube, surface area = 6 * side * side
fn cube_surface_area(side: f64) -> f64 {
    6.0 * side * side
}

// Cylinder, volume = pi * radius * radius * height
fn cylinder_volume(radius: f64, height: f64) -> f64 {
    PI * radius * radius * height
}

fn main() {
    loop {
        println!("\n=== Shape Calculator ===");
        println!("1. Trapezium (area)");
        println!("2. Rhombus (area)");
        println!("3. Parallelogram (area)");
        println!("4. Cube (surface area)");
        println!("5. Cylinder (volume)");
        println!("6. Quit");
        print!("Choose a shape (1-6): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        if io::stdin().read_line(&mut choice).is_err() {
            println!("Could not read input.");
            continue;
        }

        match choice.trim() {
            "1" => {
                let height = read_number("Enter height: ");
                let base1 = read_number("Enter base1: ");
                let base2 = read_number("Enter base2: ");
                println!(
                    "Area of trapezium = {:.2}",
                    trapezium_area(height, base1, base2)
                );
            }
            "2" => {
                let d1 = read_number("Enter diagonal1: ");
                let d2 = read_number("Enter diagonal2: ");
                println!("Area of rhombus = {:.2}", rhombus_area(d1, d2));
            }
            "3" => {
                let base = read_number("Enter base: ");
                let altitude = read_number("Enter altitude: ");
                println!(
                    "Area of parallelogram = {:.2}",
                    parallelogram_area(base, altitude)
                );
            }
            "4" => {
                let side = read_number("Enter side: ");
                println!("Surface area of cube = {:.2}", cube_surface_area(side));
            }
            "5" => {
                let radius = read_number("Enter radius: ");
                let height = read_number("Enter height: ");
                println!(
                    "Volume of cylinder = {:.2}",
                    cylinder_volume(radius, height)
                );
            }
            "6" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, please enter a number from 1 to 6."),
        }
    }
}
