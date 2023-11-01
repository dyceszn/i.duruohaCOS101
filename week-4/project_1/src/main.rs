use std::io;

fn main() {
    //define variiables

   /* 
    let d = 80.00 * 1.609; //convert to kilometers
    let t = 2.00;

    let s = d / t;
    println!("The car goes {} kmph",s );

    let d = 120.00 * 1.609; //convert to kilometers
    let t = 4.00;

    let s = d / t;
    println!("The car goes {} kmph",s );
    
    */
    let mut distance = String::new();
    println!("Enter distance in miles:");
    io::stdin().read_line(&mut distance).expect("Not a valid string");
    let d:f32 = distance.trim().parse().expect("Not a valid number");

    let mut time = String::new();
    println!("Enter Time in hours:");
    io::stdin().read_line(&mut time).expect("Not a valid string");
    let t:f32 = time.trim().parse().expect("Not a valid number");

    // x is distance in kilometers 

    let x = d * 1.609;
    let s = x / t;

    println!("The Speed is {} Kmph",s);
}
