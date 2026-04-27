use std::io::{self, Error};

#[derive(Debug)]
#[allow(dead_code)]
struct BankAccount {
    first_name: String,
    last_name: String,
    balance: f64,
    account_number: String,
}
fn main() {
    let person_1 = BankAccount::new(
        "benjamin".to_string(),
        "carson".to_string(),
        1_000_000.0,
        "234-235-542".to_string(),
    );
    println!("{:#?}", person_1);
    let person_collapsed = person_1.expect("failed to handle person_1");

    person_collapsed.display_information();
    // let (bank_account,_)=person_1.display_information();
}

fn user_query(prompt: &str) -> Result<String, io::Error> {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input)?;
    if user_input.trim().is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "buffer cannot be empty",
        ));
    }
    Ok(user_input)
}

fn atoi(prompt: &str) -> Result<i32, io::Error> {
    let buffer = user_query(prompt)?;
    let buffer = buffer
        .trim()
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "not a number"))?;
    Ok(buffer)
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl BankAccount {
    fn new(
        first_name: String,
        last_name: String,
        balance: f64,
        account_number: String,
    ) -> Result<Self, io::Error> {
        // let first_name=first_name.split_whitespace().next().unwrap_or("");
        // let first_name=first_name.split_whitespace().next().unwrap_or("");
        if first_name.is_empty()
            || last_name.is_empty()
            || balance <= 0.0
            || account_number.is_empty()
        {
            // return Err(String::from("argument empty, all arguments must be handled"))
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "argument empty, all arguments must be handled",
            ));
        }
        // (Self{
        //     first_name, last_name, balance, account_number
        // },"Ok".to_string())
        Ok(Self {
            first_name,
            last_name,
            balance,
            account_number,
        })
        // fn new(first_name:String,last_name:String,balance:f64,account_number:String)->(Self,String){
        // // fn new(first_name:String,last_name:String,balance:f64,account_number:String)->Result<Self,io::Error> {
        //     // let first_name=first_name.split_whitespace().next().unwrap_or("");
        //     // let first_name=first_name.split_whitespace().next().unwrap_or("");
        //     // if first_name.is_empty() || last_name.is_empty() || balance<=0.0|| account_number.is_empty(){
        //     //     return Err(String::from("argument empty, all arguments must be handled"))
        //     //     // return Err(io::Error::new(io::ErrorKind::InvalidInput, "argument empty, all arguments must be handled"))
        //     // }
        //     (Self{
        //         first_name, last_name, balance, account_number
        //     },"Ok".to_string())
        //     // Ok(Self{
        //     //     first_name, last_name, balance, account_number
        //     // })
    }
    // fn new(first_name:String,last_name:String,balance:f64,account_number:String)->Self {
    //     Self{
    //         first_name, last_name, balance, account_number
    //     }
    //
    //
    // }

    fn return_amount(&self) {
        println!("account balance:{:.1?}", self.balance);
    }

    fn deposit_money(&mut self, amount: f64) -> Result<f64, io::Error> {
        BankAccount::return_amount(&self);
        self.balance += amount;
        println!("current amount:{:?}", self.balance);
        Ok(self.balance)
    }

    fn withdraw_money(&mut self, amount: f64) -> Result<f64, io::Error> {
        BankAccount::return_amount(&self);
        if amount <= 0.0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "amount cannot be less than 0",
            ));
            // return Err()
        } else if amount > self.balance {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "amount requested is more than amount in in account",
            ));
        }
        self.balance -= amount;
        println!("current amount:{:?}", self.balance);
        Ok(self.balance)
    }

    fn display_information(&self) {
        println!(
            "first name:{}\nlast name:{}\nbalance:{}\naccount number:{}\n",
            self.first_name, self.last_name, self.balance, self.account_number
        );
    }
}
