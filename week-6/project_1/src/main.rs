
use std::io;

fn main() {
    // Display the menu
    println!("========================================");
    println!("          PROJECT: RESTAURANT MENU      ");
    println!("========================================");
    println!(" [P] Poundo Yam / Edinkaiko Soup  - N3,200");
    println!(" [F] Fried Rice & Chicken         - N3,000");
    println!(" [A] Amala & Ewedu Soup           - N2,500");
    println!(" [E] Eba & Egusi Soup             - N2,000");
    println!(" [W] White Rice & Stew            - N2,500");
    println!("========================================");

    // Read food type choice
    println!("Enter the letter for your food choice (P, F, A, E, W):");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice = choice.trim().to_uppercase();

    // Read quantity
    println!("Enter the quantity:");
    let mut quantity_str = String::new();
    io::stdin().read_line(&mut quantity_str).expect("Failed to read input");
    let quantity: u32 = match quantity_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid quantity entered.");
            return;
        }
    };

    // Determine unit price
    let unit_price = match choice.as_str() {
        "P" => 3200.0,
        "F" => 3000.0,
        "A" => 2500.0,
        "E" => 2000.0,
        "W" => 2500.0,
        _ => {
            println!("Invalid food choice selected.");
            return;
        }
    };

    // Compute total charge
    let mut total = unit_price * (quantity as f64);
    println!("\n--- Order Summary ---");
    println!("Subtotal: N{:.2}", total);

    // Apply 5% discount if total is greater than N10,000
    if total > 10000.0 {
        let discount = total * 0.05;
        total -= discount;
        println!("Discount (5% applied): -N{:.2}", discount);
    }

    println!("Final Total Charge: N{:.2}", total);
}

