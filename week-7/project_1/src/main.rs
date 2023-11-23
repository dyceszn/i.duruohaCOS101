
use std::io;

fn equation_solver(){

    //selecting equation

    let mut input = String::new();
    println!("Select your equation:\nInput 1, For area of trapezium\nInput 2, For area of Rhombus\nInput 3, For area of parallelogram\nInput 4, For area of a cube\nInput 5, For volume of a cylinder");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let equ:i32 = input.trim().parse().expect("Invalid input");


    if equ == 1 { //area of trapezium

        let mut input1 = String::new();
        let mut input2 = String::new();
        let mut input3 = String::new();

        println!("Enter value for height:");
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let h:f64 = input1.trim().parse().expect("Invalid input");

        println!("Enter value for first base:");
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let base1:f64 = input2.trim().parse().expect("Invalid input");

        println!("Enter value for second base:");
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let base2:f64 = input3.trim().parse().expect("Invalid input");

        let area_tr = h / 2.0 * (base1 + base2);
        println!("The  area of the trapezium is {}",area_tr);
    }
    else if equ == 2 { //area of Rhombus

        let mut input4 = String::new();
        let mut input5 = String::new();

        println!("Enter length of first diagonal:");
        io::stdin().read_line(&mut input4).expect("Failed to read input");
        let diagonal1:f64 = input4.trim().parse().expect("Invalid input");

        println!("Enter length of second diagonal:");
        io::stdin().read_line(&mut input5).expect("Failed to read input");
        let diagonal2:f64 = input5.trim().parse().expect("Invalid input");

        let area_r = (diagonal1 * diagonal2) / 2.0;
        println!("The area of the Rhombus is {}",area_r);
    }
    else if equ == 3 { //area of parallelogram

        let mut input6 = String::new();
        let mut input7 = String::new();

        println!("Enter length of first diagonal:");
        io::stdin().read_line(&mut input6).expect("Failed to read input");
        let base:f64 = input6.trim().parse().expect("Invalid input");

        println!("Enter length of second diagonal:");
        io::stdin().read_line(&mut input7).expect("Failed to read input");
        let altitude:f64 = input7.trim().parse().expect("Invalid input");

        let area_p = base * altitude;
        println!("The area of the parallelogram is {}",area_p);
    }
    else if equ == 4 { //area of a cube

        let mut input8 = String::new();

        println!("Enter length of cube:");
        io::stdin().read_line(&mut input8).expect("Failed to read input");
        let l:f64 = input8.trim().parse().expect("Invalid input");

        let x = f64::powf(l,2.0);
        let area_c = 6.0 * x;
        println!("The area of the is {}",area_c);
    }
    else if equ == 5  { //volume of a cylinder

        let mut input9 = String::new();
        let mut input10 = String::new();

        println!("Enter radius of cyliner:");
        io::stdin().read_line(&mut input9).expect("Failed to read input");
        let r:f64 = input9.trim().parse().expect("Invalid input");

        println!("Enter height of cylinder:");
        io::stdin().read_line(&mut input10).expect("Failed to read input");
        let h:f64 = input10.trim().parse().expect("Invalid input");

        let vol_c = 3.14 * r * h;
        println!("The volume of the is {}",vol_c);
    }
    else{ //
        println!("OOPS! Something went wrong");
    }

}


fn main() {

    println!("This program allows you to select an equation and solve!");
    equation_solver()
}
