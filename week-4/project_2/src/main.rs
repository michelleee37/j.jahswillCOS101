use std::io;

fn main() {
    let mut input = String::new();

    println!("Is the employee experienced? (yes/no):");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let is_experienced = input.trim().to_lowercase() == "yes";
    input.clear();

    if is_experienced {
        println!("Enter employee age:");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let age: u32 = input.trim().parse().unwrap_or(0);

        let incentive = if age >= 40 {
            1_560_000
        } else if age >= 30 {
            1_480_000
        } else if age < 28 {
            1_300_000
        } else {
            // Default fallback for ages 28 and 29 if not explicitly in table
            1_300_000
        };

        println!("Annual Incentive: N{}", incentive);
    } else {
        println!("Annual Incentive: N100,000");
    }
}