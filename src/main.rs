use std::io::{self, Error, Write};

#[derive(Debug)]
#[allow(dead_code)]
struct LoggedUser{
    first_name:String,
    last_name:String,
    time:String,
}
struct BankAccount {
    first_name: String,
    last_name: String,
    balance: f64,
    account_number: String,
}
fn main() {
    println!("Welcome to Simple Bank!");
    println!("=======================");
    print!("let's set up your account");
    std::io::stdout().flush().unwrap_or_default();
    let first_name=user_query("enter first name:").unwrap_or("cannot collect user first name".to_string());
    let last_name=user_query("enter last name:").unwrap_or("cannot collect user last name".to_string());
    let balance=atof64("enter balance:").unwrap_or(0.0);
    let account_number=user_query("enter account number:").unwrap_or("cannot collect user account".to_string());
    let person_1=BankAccount::new(first_name, last_name, balance, account_number);

    // println!("\nWhat would you like to do?");
    // println!("1. Check Balance");
    // println!("2. Make Deposit");
    // println!("3. Make Withdrawal");
    // println!("4. View Account Info");
    // println!("5. Exit");
    // println!("Enter your choice (1-5):");

    let user_bank_query=atoi("\n
    What would you like to do?\n
        1. Check Balance\n
        2. Make Deposit\n
        3. Make Withdrawa\n
        4. View Account Info\n
        5. Exit
        Enter your choice (1-5):").unwrap_or_default();


        match user_bank_query{
            0| 5=>{
                print!("exit,program ended gracefully")
            },
            1=>{
                let person_1_collaspsed=person_1.return
            }

        }

    // let person_1 = BankAccount::new(
    //     "benjamin".to_string(),
    //     "carson".to_string(),
    //     1_000_000.0,
    //     "234-235-542".to_string(),
    // );
    // println!("{:#?}", person_1);
    // let person_collapsed = person_1.expect("failed to handle person_1");
    //
    // person_collapsed.display_information();
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

fn atof64(prompt: &str) -> Result<f64, io::Error> {
    let buffer = user_query(prompt)?;
    let buffer = buffer
        .trim()
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "not a number"))?;
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

    fn check_amount(&self) {
        println!("account balance:{:.1?}", self.balance);
    }

    fn deposit_money(&mut self, amount: f64) -> Result<f64, io::Error> {
        BankAccount::check_amount(&self);
        self.balance += amount;
        println!("current amount:{:?}", self.balance);
        Ok(self.balance)
    }

    fn withdraw_money(&mut self, amount: f64) -> Result<f64, io::Error> {
        BankAccount::check_amount(&self);
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
