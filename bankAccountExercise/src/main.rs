// main.rs
mod bank_account;

use bank_account::BankAccount;

fn main() {
    // Create a new bank account with an initial balance of 500
    let mut account = BankAccount::new(500.0);
    println!("Initial balance: {:.2}", account.balance());

    // Deposit money
    account.deposit(150.0);
    println!("After depositing 150: {:.2}", account.balance());

    // Withdraw money
    account.withdraw(100.0);
    println!("After withdrawing 100: {:.2}", account.balance());

    account.withdraw(600.0);
    println!("After trying to withdraw 600: {:.2}", account.balance());

    // Apply 5% interest
    account.apply_interest(0.05);
    println!("After applying 5% interest: {:.2}", account.balance());

    // Try depositing a negative amount
    account.deposit(-50.0);
    println!("After trying to deposit -50: {:.2}", account.balance());

    // Final balance check
    println!("Final balance: {:.2}", account.balance());
}
