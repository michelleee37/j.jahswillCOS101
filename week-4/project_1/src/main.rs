use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter value for a:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let a: f64 = input.trim().parse().expect("Please enter a valid number");
    input.clear();

    println!("Enter value for b:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let b: f64 = input.trim().parse().expect("Please enter a valid number");
    input.clear();

    println!("Enter value for c:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let c: f64 = input.trim().parse().expect("Please enter a valid number");

    // Calculate the discriminant
    let d = b * b - 4.0 * a * c;

    if d > 0.0 {
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);
        println!("Two distinct real roots: {} and {}", root1, root2);
    } else if d == 0.0 {
        let root = -b / (2.0 * a);
        println!("Exactly one real root: {}", root);
    } else {
        println!("No real roots (discriminant is negative).");
    }
}