trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            Err("Deposite amount must be greater than zero.".to_string())
        }else {
            self.balance += amount;
            Ok(())
        }
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            Err("Withdrawal amount must be greater than zero.".to_string())
        }else if amount > self.balance {
            Err("Insufficient funds.".to_string())
        }else {
            self.balance -= amount;
            Ok(())
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main(){
    let mut account1 = BankAccount{
        account_number: 1001,
        holder_name: "Nauman".to_string(),
        balance: 5000.0,
    };

    let mut account2 = BankAccount{
        account_number: 1002,
        holder_name: "Rais".to_string(),
        balance: 1000.0,
    };

    match account1.deposit(2000.0) {
        Ok(_) => println!("Deposite successful"),
        Err(err) => println!("Deposite failed: {}", err),
    }

    match account2.withdraw(1050.0) {
        Ok(_) => println!("Withdrawal successful"),
        Err(err) => println!("Withdrawal failed: {}", err),
    }

    println!("Account number: {}, {}'s Account Balance: ${:.2}", account1.account_number, account1.holder_name, account1.balance);
    println!("Account number: {}, {}'s Account Balance: ${:.2}", account2.account_number, account2.holder_name, account2.balance);

}