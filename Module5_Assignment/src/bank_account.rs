// Assignment module 5.
// bank_account template.

#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}


impl BankAccount {
    // to create new account with balance
    pub fn new(initial_balance: f64) -> BankAccount {
        BankAccount {
            balance: initial_balance,
        }
    }

    // deposit money
    pub fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    //withdraws money from the account
    pub fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    // returns balance
    pub fn balance(&self) -> f64 {
        // Implement this method
        self.balance
    }

    // Bonus: Implement and test an apply_interest method that increases the balance by a given interest rate.
    pub fn apply_interest(&mut self, rate: f64) {
        if rate > 0.0 {
            self.balance += self.balance * rate;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ESPILON: f64 = 1e-10;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
        let account = BankAccount::new(115.0);
        assert!((account.balance() - 115.0).abs() < ESPILON);
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
        let mut account = BankAccount::new(115.0);
        account.deposit(20.0);

        assert!((account.balance() - 135.0).abs() < ESPILON);
    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let mut account = BankAccount::new(115.0);
        account.withdraw(15.0);

        assert!((account.balance() - 100.0).abs() < ESPILON);
    }

    // Add more tests here
    // test to apply interest
    #[test]
    fn test_interest() {
        let mut account = BankAccount::new(115.0);
        account.apply_interest(0.08); //8%

        assert!((account.balance() - 124.2).abs() < ESPILON);
    }

    // test withdraw too much
    #[test]
    fn test_withdraw_over() {
        let mut account = BankAccount::new(115.0);
        account.withdraw(200.0);

        assert!((account.balance() - 115.0).abs() < ESPILON);
    }

    // trsting negative deposit
    #[test]
    fn test_deposit_neg() {
        let mut account = BankAccount::new(115.0);
        account.deposit(-50.0);

        assert!((account.balance() - 115.0).abs() < ESPILON);
    }

    // testing negative withdraw
    #[test]
    fn test_withdraw_neg() {
        let mut account = BankAccount::new(115.0);
        account.withdraw(-15.0);

        assert!((account.balance() - 115.0).abs() < ESPILON);
    }
}
