use std::io;
use std::io::Read;

fn main() {

    
    println!("GLOBACOM DATABASE MANAGEMENT SYSTEM");

    let mut input1 = String::new();
    let mut input2 = String::new();


    println!("Enter your name:"); //For username
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let name:String = input1.trim().parse().expect("Invalid input");

    println!("\nSelect Position:\nType 1 For Administrator\nType 2 For Project Manager\nType 3 For Employee\nType 4 For Customer\nType 5 For Vendor"); //For Post
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let post:i32 = input2.trim().parse().expect("Invalid input");


    if post == 1 {
        admin();
    } else if post == 2 {
        projectman();
    } else if post == 3 {
        employee();
    } else if post == 4 {
        customer();
    } else if post == 5 {
        vendor();
    } else {
        print!("Sorry, Invalid Input");
    }

}


fn admin() {

    let mut file = std::fs::File::open("globacom_db.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);

}

fn projectman() {

    let mut file = std::fs::File::open("project_tbl.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);

}

fn employee() {

    let mut file = std::fs::File::open("staff_tbl.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);

}

fn customer() {

    let mut file = std::fs::File::open("customer_tbl.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);

}

fn vendor() {

    let mut file = std::fs::File::open("dataplan_tbl.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);

}
