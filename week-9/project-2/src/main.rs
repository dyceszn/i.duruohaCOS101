use std::io;
use std::io::Write;
use std::io::Read;

fn main() {
    
    println!("\nWelcome to the PAU SIMS\n");


    let header:[&str;4] = ["NAME","MATRIC NUMBER","DEPARTMENT","LEVEL\n"];


    //convert array to string

    let head:String = header.join(" | ");



    //Writing array to file
 
    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all(head.as_bytes()).expect("write failed");


    //Get students inputs

    let mut usersinput : Vec<String> = Vec::new();

    let mut x = 1;

    while x == 1 {

        let mut input1 = String::new();
        let mut input2 = String::new();
        let mut input3 = String::new();
        let mut input4 = String::new();
        let mut input5 = String::new();


        println!("Enter your name:");
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let name:String = input1.trim().parse().expect("Invalid input");

        println!("Enter your Matric Number:");
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let matric:String = input2.trim().parse().expect("Invalid input");

        println!("Enter your Department");
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let dept:String = input3.trim().parse().expect("Invalid input");

        println!("Enter your Level:");
        io::stdin().read_line(&mut input4).expect("Failed to read input");
        let level:String = input4.trim().parse().expect("Invalid input");


        usersinput.push(name.to_string());
        usersinput.push(matric.to_string());
        usersinput.push(dept.to_string());
        usersinput.push(level.to_string());


        println!("Student processed succesfully");
        println!("Would you like to add another student? Input 1 for Yes, Input 0 for No");
        io::stdin().read_line(&mut input5).expect("Failed to read input");
        let x:i32 = input5.trim().parse().expect("Invalid input");

        if x == 0 {
            break;
        }

}

    //convert vector array to string with a newline after every 4 elements

    let usersinputstr: String = usersinput.chunks(4).map(|chunk| chunk.join(" | ")).collect::<Vec<String>>().join("\n");

    //Writing array to file

    file.write_all(usersinputstr.as_bytes()).expect("write failed");

    //displaying

    println!("File write successfully!\n");

    println!("PAU SIMS\n");

    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();


    println!("{}",contents);


}