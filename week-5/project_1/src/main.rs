
use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter value for a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f64 = input1.trim().parse().expect("Not a valid number");

    println!("Enter value for b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f64 = input2.trim().parse().expect("Not a valid number");

    println!("Enter value for c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f64 = input3.trim().parse().expect("Not a valid number");

// discriminant is b2 - 4ac
    let x = f64::powf(b,2.0);
    let discriminant = x - (4.0 * a * c);

// adding if elements

     if discriminant < 0.0 { //negative
        println!("There are no real roots");
    }

    else if discriminant > 0.0 || discriminant == 0.0 { //positive
        //implement remaining formula
        let y = discriminant.sqrt();
        let quadratic1 = ( - b + y)/ 2.0 * a;
        let quadratic2 = ( - b - y)/ 2.0 * a;

        println!("The root(s) are {} {}",quadratic1,quadratic2);
    }
    else {
        println!("Impossible");
    }

}
