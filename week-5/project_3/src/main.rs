
use std::io;

fn main() {

    //Displays menu

    println!("MENU");
    println!("P: Poundo Yam/Edinkaiko soup = N3200");
    println!("F: Fried rice and chicken = N3000");
    println!("A: Amala and Ewedu Soup = N2500");
    println!("E: Eba and Egusi soup = N2000");
    println!("W: White rice and Stew = N2500");

    //User inputs order

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();

    println!("Enter quantity for P(If you dont want any, input 0): ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let p:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter quantity for F(If you dont want any, input 0): ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let f:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter quantity for P(If you dont want any, input 0): ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let a:f32 = input3.trim().parse().expect("Not a valid number");

    println!("Enter quantity for P(If you dont want any, input 0): ");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    let e:f32 = input4.trim().parse().expect("Not a valid number");

    println!("Enter quantity for P(If you dont want any, input 0): ");
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let w:f32 = input5.trim().parse().expect("Not a valid number");



    //Formular for individual food items

     let p1 = p * 3200.0;
     let f1 = f * 3000.0;
     let a1 = a * 2500.0;
     let e1 = e * 2000.0;
     let w1 = w * 2500.0;

    //Declaring Total order

    let total_order:f32 = p1 + f1 + a1 + e1 + w1;

    //Conclusion

    if total_order > 10_000.0 {
    let discounted = 0.95 * total_order;
        println!("You have received a 5% discount. Your discounted price is {} naira",discounted);
    } 
    else {
         println!("Yor total bill is {} naira",total_order);
    }
}
