use std::io;

fn main() {
    // Get the principal amount from the user
    println!("Enter the principal amount: ");
    let mut principal = String::new();
    io::stdin().read_line(&mut principal).expect("Failed to read line");
    let mut principal: f64 = principal.trim().parse().expect("Please enter a valid number");

    // Get the monthly interest rate from the user
    println!("Enter the monthly interest rate (in decimal form, e.g., 0.02 for 2%): ");
    let mut monthly_interest_rate = String::new();
    io::stdin().read_line(&mut monthly_interest_rate).expect("Failed to read line");
    let monthly_interest_rate: f64 = monthly_interest_rate.trim().parse().expect("Please enter a valid number");

    // Calculate the annual interest rate
    let annual_interest_rate = monthly_interest_rate * 12.0 * 100.0; // Convert to annual and percentage

    // Get the loan term in months from the user
    println!("Enter the number of months for the loan term: ");
    let mut months = String::new();
    io::stdin().read_line(&mut months).expect("Failed to read line");
    let months: u32 = months.trim().parse().expect("Please enter a valid number");

    let mut total_interest = 0.0;

    for _ in 0..months {
        let interest = principal * monthly_interest_rate;
        total_interest += interest;
        principal += interest; // Update the principal after charging interest for the month
    }

    println!("Total interest earned over {} months: ${:.2}", months, total_interest);
    println!("Annual interest rate: {:.2}%", annual_interest_rate);
}
