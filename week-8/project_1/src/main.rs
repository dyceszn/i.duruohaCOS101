
use std::io;

fn main() {


    let mut inputo = String::new();
    println!("Enter your office:\nFor Office admin, input 1\nFor academic, input 2\nFor lawyer, input 3\nFor teacher, input 4");
    io::stdin().read_line(&mut inputo).expect("Failed to read input");
    let office:i32 = inputo.trim().parse().expect("Invalid input");

    if office == 1 {
        officeadmin();
    }
    else if office == 2 {
        academic();
    }
    else if office == 3 {
        lawyer();
    }
    else if office == 4 {
        teacher();
    }
    else {
        println!("OOPS");
    }


}


fn officeadmin() {

    let mut aps12 : Vec<String> = Vec::new();
    let mut aps35 : Vec<String> = Vec::new();
    let mut aps58 : Vec<String> = Vec::new();
    let mut el1810 : Vec<String> = Vec::new();
    let mut el21013 : Vec<String> = Vec::new();
    let mut ses : Vec<String> = Vec::new();

    let mut inputn = String::new();
    println!("Enter your name:");
    io::stdin().read_line(&mut inputn).expect("Failed to read input");
    let name:String = inputn.trim().parse().expect("Invalid input");
    
    let mut inputl = String::new();

    println!("what is your role:\nFor Intern, Input 1\nFor Administrator, input 2\nFor Senior Administrator,input 3\nFor Office Manager, input 4\nFor Director, input 5\nFor CEO, input 6");
    io::stdin().read_line(&mut inputl).expect("Failed to read input");
    let adminstat:i32 = inputl.trim().parse().expect("Invalid input");

    let mut inputy = String::new();
    println!("Enter your years of experience:");
    io::stdin().read_line(&mut inputy).expect("Failed to read input");
    let yoe:i32 = inputy.trim().parse().expect("Invalid input");

    if adminstat == 1 && yoe <=2 && yoe >= 1{
        aps12.push(name.to_string());
        println!("You have been Successfully added!");
        println!("APS1-2: {:?}\nAPS3-5: {:?}\nAPS5-8: {:?}\nEL1 8-10: {:?}\nEL2 10-13: {:?}\nSES: {:?}",aps12,aps35,aps58,el1810,el21013,ses);
    }
    else if adminstat == 2 && yoe <= 5 && yoe >= 3{
        aps35.push(name.to_string());
        println!("You have been Successfully added!");
        println!("APS1-2: {:?}\nAPS3-5: {:?}\nAPS5-8: {:?}\nEL1 8-10: {:?}\nEL2 10-13: {:?}\nSES: {:?}",aps12,aps35,aps58,el1810,el21013,ses);
    }
    else if adminstat == 3 && yoe <= 8 && yoe > 5{
        aps58.push(name.to_string());
        println!("You have been Successfully added!");
        println!("APS1-2: {:?}\nAPS3-5: {:?}\nAPS5-8: {:?}\nEL1 8-10: {:?}\nEL2 10-13: {:?}\nSES: {:?}",aps12,aps35,aps58,el1810,el21013,ses);
    }
    else if adminstat == 4 && yoe <= 10 && yoe > 8{
        el1810.push(name.to_string());
        println!("You have been Successfully added!");
        println!("APS1-2: {:?}\nAPS3-5: {:?}\nAPS5-8: {:?}\nEL1 8-10: {:?}\nEL2 10-13: {:?}\nSES: {:?}",aps12,aps35,aps58,el1810,el21013,ses);
    }
    else if adminstat == 5 && yoe <= 13 && yoe >= 11{
        el21013.push(name.to_string());
        println!("You have been Successfully added!");
        println!("APS1-2: {:?}\nAPS3-5: {:?}\nAPS5-8: {:?}\nEL1 8-10: {:?}\nEL2 10-13: {:?}\nSES: {:?}",aps12,aps35,aps58,el1810,el21013,ses);
    }
    else if adminstat == 6 && yoe > 13{
        ses.push(name.to_string());
        println!("You have been Successfully added!");
        println!("APS1-2: {:?}\nAPS3-5: {:?}\nAPS5-8: {:?}\nEL1 8-10: {:?}\nEL2 10-13: {:?}\nSES: {:?}",aps12,aps35,aps58,el1810,el21013,ses);
    }
    else {
        println!("Something went wrong");
    }

}




fn academic() {

    let mut aps12 : Vec<String> = Vec::new();
    let mut aps35 : Vec<String> = Vec::new();
    let mut aps58 : Vec<String> = Vec::new();
    let mut el1810 : Vec<String> = Vec::new();
    let mut el21013 : Vec<String> = Vec::new();
    let mut ses : Vec<String> = Vec::new();

        let mut inputn = String::new();
    println!("Enter your name:");
    io::stdin().read_line(&mut inputn).expect("Failed to read input");
    let name:String = inputn.trim().parse().expect("Invalid input");
    
    let mut inputl = String::new();

    println!("what is your role:\nFor - , Input 1\nFor Research Assistant, input 2\nFor PhD Candidate, input 3\nFor Post Doc Researcher, input 4\nFor Senior Lecturer, input 5\nFor Dean, input 6");
    io::stdin().read_line(&mut inputl).expect("Failed to read input");
    let acadstat:i32 = inputl.trim().parse().expect("Invalid input");

    let mut inputy = String::new();
    println!("Enter your years of experience:");
    io::stdin().read_line(&mut inputy).expect("Failed to read input");
    let yoe:i32 = inputy.trim().parse().expect("Invalid input");


    if acadstat == 1 && yoe <= 2 && yoe >= 1{
        aps12.push(name.to_string());
        println!("You have been Successfully added!");
        println!("APS1-2: {:?}\nAPS3-5: {:?}\nAPS5-8: {:?}\nEL1 8-10: {:?}\nEL2 10-13: {:?}\nSES: {:?}",aps12,aps35,aps58,el1810,el21013,ses);
    }
    else if acadstat == 2 && yoe <= 5 && yoe >= 3{
        aps35.push(name.to_string());
        println!("You have been Successfully added!");
        println!("APS1-2: {:?}\nAPS3-5: {:?}\nAPS5-8: {:?}\nEL1 8-10: {:?}\nEL2 10-13: {:?}\nSES: {:?}",aps12,aps35,aps58,el1810,el21013,ses);
    }
    else if acadstat == 3 && yoe <= 8 && yoe > 5{
        aps58.push(name.to_string());
        println!("You have been Successfully added!");
        println!("APS1-2: {:?}\nAPS3-5: {:?}\nAPS5-8: {:?}\nEL1 8-10: {:?}\nEL2 10-13: {:?}\nSES: {:?}",aps12,aps35,aps58,el1810,el21013,ses);
    }
    else if acadstat == 4 && yoe <= 10 && yoe > 8{
        el1810.push(name.to_string());
        println!("You have been Successfully added!");
        println!("APS1-2: {:?}\nAPS3-5: {:?}\nAPS5-8: {:?}\nEL1 8-10: {:?}\nEL2 10-13: {:?}\nSES: {:?}",aps12,aps35,aps58,el1810,el21013,ses);
    }
    else if acadstat == 5 && yoe <= 13 && yoe >= 11{
        el21013.push(name.to_string());
        println!("You have been Successfully added!");
        println!("APS1-2: {:?}\nAPS3-5: {:?}\nAPS5-8: {:?}\nEL1 8-10: {:?}\nEL2 10-13: {:?}\nSES: {:?}",aps12,aps35,aps58,el1810,el21013,ses);
    }
    else if acadstat == 6 && yoe > 13{
        ses.push(name.to_string());
        println!("You have been Successfully added!");
        println!("APS1-2: {:?}\nAPS3-5: {:?}\nAPS5-8: {:?}\nEL1 8-10: {:?}\nEL2 10-13: {:?}\nSES: {:?}",aps12,aps35,aps58,el1810,el21013,ses);
    }
    else {
        println!("Something went wrong");
    }

}





fn teacher() {

    let mut aps12 : Vec<String> = Vec::new();
    let mut aps35 : Vec<String> = Vec::new();
    let mut aps58 : Vec<String> = Vec::new();
    let mut el1810 : Vec<String> = Vec::new();
    let mut el21013 : Vec<String> = Vec::new();
    let mut ses : Vec<String> = Vec::new();

    let mut inputn = String::new();
    println!("Enter your name:");
    io::stdin().read_line(&mut inputn).expect("Failed to read input");
    let name:String = inputn.trim().parse().expect("Invalid input");
    
    let mut inputl = String::new();

    println!("what is your role:\nFor Placement, Input 1\nFor Classroom Teacher, input 2\nFor Senior Teacher,input 3\nFor Leading Teacher, input 4\nFor Deputy Principal, input 5\nFor Principal, input 6");
    io::stdin().read_line(&mut inputl).expect("Failed to read input");
    let teachstat:i32 = inputl.trim().parse().expect("Invalid input");

    let mut inputy = String::new();
    println!("Enter your years of experience:");
    io::stdin().read_line(&mut inputy).expect("Failed to read input");
    let yoe:i32 = inputy.trim().parse().expect("Invalid input");


    if teachstat == 1 && yoe <= 2 && yoe >= 1{
        aps12.push(name.to_string());
        println!("You have been Successfully added!");
        println!("APS1-2: {:?}\nAPS3-5: {:?}\nAPS5-8: {:?}\nEL1 8-10: {:?}\nEL2 10-13: {:?}\nSES: {:?}",aps12,aps35,aps58,el1810,el21013,ses);
    }
    else if teachstat == 2 && yoe <= 5 && yoe >= 3{
        aps35.push(name.to_string());
        println!("You have been Successfully added!");
        println!("APS1-2: {:?}\nAPS3-5: {:?}\nAPS5-8: {:?}\nEL1 8-10: {:?}\nEL2 10-13: {:?}\nSES: {:?}",aps12,aps35,aps58,el1810,el21013,ses);
    }
    else if teachstat == 3 && yoe <= 8 && yoe > 5{
        aps58.push(name.to_string());
        println!("You have been Successfully added!");
        println!("APS1-2: {:?}\nAPS3-5: {:?}\nAPS5-8: {:?}\nEL1 8-10: {:?}\nEL2 10-13: {:?}\nSES: {:?}",aps12,aps35,aps58,el1810,el21013,ses);
    }
    else if teachstat == 4 && yoe <= 10 && yoe > 8{
        el1810.push(name.to_string());
        println!("You have been Successfully added!");
        println!("APS1-2: {:?}\nAPS3-5: {:?}\nAPS5-8: {:?}\nEL1 8-10: {:?}\nEL2 10-13: {:?}\nSES: {:?}",aps12,aps35,aps58,el1810,el21013,ses);
    }
    else if teachstat == 5 && yoe <= 13 && yoe >= 11{
        el21013.push(name.to_string());
        println!("You have been Successfully added!");
        println!("APS1-2: {:?}\nAPS3-5: {:?}\nAPS5-8: {:?}\nEL1 8-10: {:?}\nEL2 10-13: {:?}\nSES: {:?}",aps12,aps35,aps58,el1810,el21013,ses);
    }
    else if teachstat == 6 && yoe > 13{
        ses.push(name.to_string());
        println!("You have been Successfully added!");
        println!("APS1-2: {:?}\nAPS3-5: {:?}\nAPS5-8: {:?}\nEL1 8-10: {:?}\nEL2 10-13: {:?}\nSES: {:?}",aps12,aps35,aps58,el1810,el21013,ses);
    }
    else {
        println!("Something went wrong");
    }

}



fn lawyer() {

    let mut aps12 : Vec<String> = Vec::new();
    let mut aps35 : Vec<String> = Vec::new();
    let mut aps58 : Vec<String> = Vec::new();
    let mut el1810 : Vec<String> = Vec::new();
    let mut el21013 : Vec<String> = Vec::new();
    let mut ses : Vec<String> = Vec::new();

    let mut inputn = String::new();
    println!("Enter your name:");
    io::stdin().read_line(&mut inputn).expect("Failed to read input");
    let name:String = inputn.trim().parse().expect("Invalid input");
    
    let mut inputl = String::new();
    println!("what is your role:\nFor Paralegal, Input 1\nFor Junior Associate, input 2\nFor Associate,input 3\nFor Senior Associate 1-2, input 4\nFor Senior Associate 3-4, input 5\nFor Partner, input 6");
    io::stdin().read_line(&mut inputl).expect("Failed to read input");
    let lawstat:i32 = inputl.trim().parse().expect("Invalid input");

    let mut inputy = String::new();
    println!("Enter your years of experience:");
    io::stdin().read_line(&mut inputy).expect("Failed to read input");
    let yoe:i32 = inputy.trim().parse().expect("Invalid input");


    if lawstat == 1 && yoe <= 2 && yoe >= 1{
        aps12.push(name.to_string());
        println!("You have been Successfully added!");
        println!("APS1-2: {:?}\nAPS3-5: {:?}\nAPS5-8: {:?}\nEL1 8-10: {:?}\nEL2 10-13: {:?}\nSES: {:?}",aps12,aps35,aps58,el1810,el21013,ses);
    }
    else if lawstat == 2 && yoe <= 5 && yoe >= 3{
        aps35.push(name.to_string());
        println!("You have been Successfully added!");
        println!("APS1-2: {:?}\nAPS3-5: {:?}\nAPS5-8: {:?}\nEL1 8-10: {:?}\nEL2 10-13: {:?}\nSES: {:?}",aps12,aps35,aps58,el1810,el21013,ses);
    }
    else if lawstat == 3 && yoe <= 8 && yoe > 5{
        aps58.push(name.to_string());
        println!("You have been Successfully added!");
        println!("APS1-2: {:?}\nAPS3-5: {:?}\nAPS5-8: {:?}\nEL1 8-10: {:?}\nEL2 10-13: {:?}\nSES: {:?}",aps12,aps35,aps58,el1810,el21013,ses);
    }
    else if lawstat == 4 && yoe <= 10 && yoe > 8{
        el1810.push(name.to_string());
        println!("You have been Successfully added!");
        println!("APS1-2: {:?}\nAPS3-5: {:?}\nAPS5-8: {:?}\nEL1 8-10: {:?}\nEL2 10-13: {:?}\nSES: {:?}",aps12,aps35,aps58,el1810,el21013,ses);
    }
    else if lawstat == 5 && yoe <= 13 && yoe >= 11{
        el21013.push(name.to_string());
        println!("You have been Successfully added!");
        println!("APS1-2: {:?}\nAPS3-5: {:?}\nAPS5-8: {:?}\nEL1 8-10: {:?}\nEL2 10-13: {:?}\nSES: {:?}",aps12,aps35,aps58,el1810,el21013,ses);
    }
    else if lawstat == 6 && yoe > 13{
        ses.push(name.to_string());
        println!("You have been Successfully added!");
        println!("APS1-2: {:?}\nAPS3-5: {:?}\nAPS5-8: {:?}\nEL1 8-10: {:?}\nEL2 10-13: {:?}\nSES: {:?}",aps12,aps35,aps58,el1810,el21013,ses);
    }
    else {
        println!("Something went wrong");
    }

}

