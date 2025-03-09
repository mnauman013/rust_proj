
// Account Traits
trait Account {
    fn deposite(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}

// struct that define bank account
struct BankAccount {
    account_number: u32,
    account_holder_name: String,
    balance: f64,
}

// implementing account trait for bank accout
impl Account for BankAccount {
    fn deposite(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
            println!("[Account {}] {} deposited {:.2}. New balance: {:.2}",
            self.account_number, self.account_holder_name, amount, self.balance);
        }else {
            println!("Deposite amount must be positive.");
        }
    }

    fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance{
            self.balance -=amount;
            println!("[Account {}] {} withdraw {:.2}. New balance: {:.2}",
            self.account_number, self.account_holder_name, amount, self.balance);
        }else {
            println!("[Account {}] Withdrawal failed. Check amount (availabe balance: {:.2})", self.account_number, self.balance);
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

// main function
fn main(){
    // first account
    let mut account1 = BankAccount{
        account_number: 12345,
        account_holder_name: String::from("Nauman"),
        balance: 20000.0,
    };
    // second account
    let mut account2 = BankAccount{
        account_number: 12346,
        account_holder_name: String::from("Ahmad"),
        balance: 1000.0,
    };

    // transactions
    account1.deposite(1000.0);
    account2.withdraw(2000.0);

    // getting balance details
    println!("{}'s final balance: {:.2}", account1.account_holder_name, account1.balance());
    println!("{}'s final balance: {:.2}", account2.account_holder_name, account2.balance());
}