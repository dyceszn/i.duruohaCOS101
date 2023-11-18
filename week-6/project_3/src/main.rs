
use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your value for n (n multiplication table): ");
        io::stdin().read_line(&mut input1).expect("Not a valid string");
        let n:i32 = input1.trim().parse().expect("Not a valid value");

    println!("To what term: (nth term) ");
        io::stdin().read_line(&mut input2).expect("Not a valid string");
        let m:i32 = input2.trim().parse().expect("Not a valid value");

    println!("Multiplication table for {} (up to {}th multiple)",n,m);

    for i in 1..m + 1 {
                println!("{} x {} = {}",n,i,n * i);
            }        


}

