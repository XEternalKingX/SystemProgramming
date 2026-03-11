// Assignment Module 5.

// Simplified Rust Struct Testing Exercise: BankAccount
// Objective
// Create a BankAccount struct with methods to deposit, withdraw, and check the balance. 
// Write tests for these methods.

mod bank_account;
use bank_account::BankAccount;


fn main() {
    // creating a new account with 115
    let mut account = BankAccount::new(115.0);

    println!("Initial balance: {}", account.balance());

    //deposit money
    account.deposit(60.0);
    println!("Balance after deposit: {}", account.balance());

    //withdraw money
    account.withdraw(20.0);
    println!("Balance after withdraw: {}", account.balance());

    // apply interest
    account.apply_interest(0.10); //10%

    println!("Balance after interest: {}", account.balance());
}
