use std::io;

fn main() {
    

    let mut usersname : Vec<String> = Vec::new();
    let mut usersage : Vec<String> = Vec::new();
    let mut usersnum : Vec<String> = Vec::new();
    let mut usersadd : Vec<String> = Vec::new();
    let mut usersskills : Vec<String> = Vec::new();
    let mut userslastwork : Vec<String> = Vec::new();
    let mut usershel : Vec<String> = Vec::new();
    let mut userscv : Vec<String> = Vec::new();
    let mut usersyoe : Vec<i32> = Vec::new();

    let mut input0 = String::new();

    println!("[Admin Only] Enter Number of Candidates:");
    io::stdin().read_line(&mut input0).expect("Failed to read input");
    let intno:i32 = input0.trim().parse().expect("Invalid input");


    for val in 0..intno {


    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();
    let mut input8 = String::new();
    let mut input9 = String::new();


    println!("Enter your full name:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let name:String = input1.trim().parse().expect("Invalid input");

    println!("Enter your Age:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let age:i32 = input2.trim().parse().expect("Invalid input");

    println!("Enter your Phone number:");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let num:String = input3.trim().parse().expect("Invalid input");

    println!("Enter your Address:");
    io::stdin().read_line(&mut input4).expect("Failed to read input");
    let add:String = input4.trim().parse().expect("Invalid input");

    println!("Enter your Special skills, if any:");
    io::stdin().read_line(&mut input5).expect("Failed to read input");
    let skills:String = input5.trim().parse().expect("Invalid input");

    println!("Enter your Last place of work, if any:");
    io::stdin().read_line(&mut input6).expect("Failed to read input");
    let lastwork:String = input6.trim().parse().expect("Invalid input");

    println!("Enter your Highest educational level:");
    io::stdin().read_line(&mut input7).expect("Failed to read input");
    let hel:String = input7.trim().parse().expect("Invalid input");

    println!("Enter your link to portfolio/CV, if any:");
    io::stdin().read_line(&mut input8).expect("Failed to read input");
    let cv:String = input8.trim().parse().expect("Invalid input");

    println!("Enter your years of experience:");
    io::stdin().read_line(&mut input9).expect("Failed to read input");
    let yoe:i32 = input9.trim().parse().expect("Invalid input");

    println!("Your details has been submitted successfully!\n");

    usersname.push(input1.to_string());
    usersage.push(input2.to_string());
    usersnum.push(input3.to_string());
    usersadd.push(input4.to_string());
    usersskills.push(input5.to_string());
    userslastwork.push(input6.to_string());
    usershel.push(input7.to_string());
    userscv.push(input8.to_string());
    usersyoe.push(yoe);


    }

    if let Some((index, &max)) = usersyoe.iter().enumerate().max_by_key(|(_, &value)| value) {
        
        println!("The Candidate with the Highest Number of Experience is...");
        println!("Candiate's name: {:?}",usersname[index]);
        println!("Candiate's age: {:?}",usersage[index]);
        println!("Candiate's number: {:?}",usersnum[index]);
        println!("Candiate's address: {:?}",usersadd[index]);
        println!("Candiate's special skills: {:?}",usersskills[index]);
        println!("Candiate's last place of work: {:?}",userslastwork[index]);
        println!("Candiate's highest education level: {:?}",usershel[index]);
        println!("Candiate's years of experience: {:?}",usersyoe[index]);
    
    }
    else {
        println!("empty vector");
    }

   

}

