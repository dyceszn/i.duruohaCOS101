
use std::io;

fn main() {


    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter years of Experience: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let exp:i32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:i32 = input2.trim().parse().expect("Not a valid number");

    if exp >= 8 && age >= 40 {
        println!("incentive is 1,560,000");
    }
    else if exp >= 8 && age >= 30 && age <40 {
        println!("incentive is 1,480,000");
    }
    else if exp >= 8 && age < 28 {
        println!("incentive is 1,300,000");
    }
    else if exp < 8 {
        println!("incentive is 100,000");
    }
    else {
        println!("incorrect input");
    }
}
