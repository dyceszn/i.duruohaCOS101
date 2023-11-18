
use std::io;

fn main() {

    //Data collection

    let mut x = 0;
   
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();

    println!("Enter your Full name: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let name:String = input1.trim().parse().expect("Not a valid string");

        println!("Enter your Email-address: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let email:String = input2.trim().parse().expect("Not a valid string");

        println!("Enter your Department: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let dept:String = input3.trim().parse().expect("Not a valid value");

        println!("Enter your State of origin: ");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    let soo:String = input4.trim().parse().expect("Not a valid value");

        println!("Please select your current level:\nFor 100lvl, input 1\nFor 200lvl, input 2\nFor 300lvl, input 3\nFor 400lvl, input 4 ");
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let lvl:i32 = input5.trim().parse().expect("Not a valid value");

        println!("Are you a current class rep? If yes input 1, If no input 0");
    io::stdin().read_line(&mut input6).expect("Not a valid string");
    let crep:i32 = input6.trim().parse().expect("Not a valid value");

        println!("Enter your CGPA: ");
    io::stdin().read_line(&mut input7).expect("Not a valid string");
    let cgpa:f32 = input7.trim().parse().expect("Not a valid value");

// Data collection done.

    if x <= 150 { // If slot no is less than 150 proceed

        if crep == 1 && lvl > 1 && cgpa > 4.0 {
            println!("Name:{}\nEmail-address:{}\nDepartment:{}\nState of origin:{}", name,email,dept,soo);
            println!("You can vote");
            x = x + 1;  // Increase slot no. by one
            println!("{}/150 slot(s) used",x);
        } else {
            println!("Sorry, you are not eligible to vote");
        }

    } else {
        println!("Available slots have been exceeded");
    }

}
