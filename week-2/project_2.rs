fn main() {
	let amount = vec! [450_000.0, 1_500_000.0, 750_000.0, 2_850_000.0, 250_000.0];
    let quantities = vec! [2.0, 1.0, 3.0, 3.0, 1.0];
    let total_money : f64 = amount.iter().sum();
    let total_items : f64 = quantities.iter().sum();
    println! ("sum of amounts is {}", total_money);
    println! ("sum of items is {}", total_items);
    let average = total_money / total_items;
        println! ("average of sales record is {}", average);
        println! ("Dear Esteemed User,");
        println! ("Thank you for using our program.");
        println! ("The sum and average of the sales record of P.M. Okeke and Sons Ltd are {} and{}.", total_money, average);
}