trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        self.balance -= amount;
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut account1 = BankAccount {
        account_number: 12345,
        holder_name: String::from("Alice"),
        balance: 1000.0,
    };

    let mut account2 = BankAccount {
        account_number: 67890,
        holder_name: String::from("Bob"),
        balance: 500.0,
    };

    account1.deposit(550.0);
    account2.withdraw(200.0);

    println!("Balance for account {}: ${}", account1.account_number, account1.balance());
    println!("Balance for account {}: ${}", account2.account_number, account2.balance());
}
