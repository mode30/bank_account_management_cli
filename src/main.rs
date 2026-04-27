use std::io;

#[derive(Debug)]
#[allow(dead_code)]
struct BankAccount{
    first_name:String,
    last_name:String,
    balance:f64,
    account_number:String,
}
fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl BankAccount {

    fn new(first_name:String,last_name:String,balance:f64,account_number:String)->Result<Self,io::Error> { 
        // let first_name=first_name.split_whitespace().next().unwrap_or("");
        // let first_name=first_name.split_whitespace().next().unwrap_or("");
        if first_name.is_empty() || last_name.is_empty() || balance<=0.0|| account_number.is_empty(){
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "argument empty, all arguments must be handled"))
        }
        Ok(Self{
            first_name, last_name, balance, account_number 
        })


    }
    // fn new(first_name:String,last_name:String,balance:f64,account_number:String)->Self { 
    //     Self{
    //         first_name, last_name, balance, account_number 
    //     }
    //
    //
    // }

    fn return_amount(&self){
        println!("account balance:{:.1?}",self.balance);
    }

    fn deposit_money(&mut self,amount:f64)->Result<f64,io::Error>{
        BankAccount::return_amount(&self);
        self.balance+=amount;
        println!("current amount:{:?}",self.balance);
        Ok(self.balance)
    }

    fn withdraw_money(&mut self,amount:f64)->Result<f64,io::Error>{
        BankAccount::return_amount(&self);
        if amount<=0.0{

            return Err(io::Error::new(io::ErrorKind::InvalidInput, "amount cannot be less than 0"))
            // return Err()
        }else if amount > self.balance{
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "amount requested is more than amount in in account"))

        }
        self.balance-=amount;
        println!("current amount:{:?}",self.balance);
        Ok(self.balance)
    }
}
