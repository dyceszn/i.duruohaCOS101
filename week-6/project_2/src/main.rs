
use std::io;

fn main() {
    

    let mut x = 0;
   
    let mut input1 = String::new();
    let mut input2 = String::new();

        println!("Enter your Full name: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let name:String = input1.trim().parse().expect("Not a valid string");

        println!("Enter number of papers published: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let papers:i32 = input2.trim().parse().expect("Not a valid string");

    let mut inc = 0;

    if x <= 500 {

        if papers > 3 && papers < 5{
            inc = 500_000;
        } 
        else if papers > 5 && papers < 10{
            inc = 800_000;
        }
        else if papers > 10{
            inc = 1_000_000;
        }
        else if papers < 3{
            inc = 1_000_000;
        }
        println!("Name: {}\nIncentive(In naira): {}",name,inc);
        x= x + 1;
        println!("No of slot(s) used:{}/500",x);
    }
    else {
        println!("No available slots remaining");
    }



}
