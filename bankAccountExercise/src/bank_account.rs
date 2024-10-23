#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        BankAccount {
            balance: initial_balance,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }

    pub fn apply_interest(&mut self, rate: f64) {
        if rate > 0.0 {
            self.balance += self.balance * rate;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_new_account() {
        let account = BankAccount::new(100.0);
        assert!((account.balance() - 100.0).abs() < EPSILON);
    }

    #[test]
    fn test_deposit() {
        let mut account = BankAccount::new(50.0);
        account.deposit(30.0);
        assert!((account.balance() - 80.0).abs() < EPSILON);
        account.deposit(-10.0);
        assert!((account.balance() - 80.0).abs() < EPSILON);
    }

    #[test]
    fn test_withdraw() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(40.0);
        assert!((account.balance() - 60.0).abs() < EPSILON);
        account.withdraw(100.0);
        assert!((account.balance() - 60.0).abs() < EPSILON);
        account.withdraw(-20.0);
        assert!((account.balance() - 60.0).abs() < EPSILON);
    }

    #[test]
    fn test_balance() {
        let account = BankAccount::new(200.0);
        assert!((account.balance() - 200.0).abs() < EPSILON);
    }

    #[test]
    fn test_edge_cases() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(150.0);
        assert!((account.balance() - 100.0).abs() < EPSILON);
        account.deposit(-50.0);
        assert!((account.balance() - 100.0).abs() < EPSILON);
        account.withdraw(-30.0);
        assert!((account.balance() - 100.0).abs() < EPSILON);
    }

    #[test]
    fn test_apply_interest() {
        let mut account = BankAccount::new(100.0);
        account.apply_interest(0.05);
        assert!((account.balance() - 105.0).abs() < EPSILON);
        account.apply_interest(0.0);
        assert!((account.balance() - 105.0).abs() < EPSILON);
        account.apply_interest(-0.05);
        assert!((account.balance() - 105.0).abs() < EPSILON);
    }
}
