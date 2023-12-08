use std::io::Write;
use std::io::Read;

fn main() {

    let arr1 = [["LAGER","STOUT","NON-ALCOHOLIC\n"], ["33 Export","Legend","Maltina\n"], ["Desperados","Turbo King","Amstel Malta\n"], ["Goldberg","Williams","Malta Gold\n"], ["Gulder","-","Fayrouz\n"], ["Heineken","-","-\n"], ["Star","-","-\n"]];

    //convert array to string

    let arraystr: String = arr1.iter().map(|row| row.join(" | ")).collect::<Vec<String>>().join("\n");

    //write to file

    let mut file = std::fs::File::create("data.txt").expect("create failed");

    file.write_all(arraystr.as_bytes()).expect("write failed");


    //Displaying

    println!("\nCreated successfully\n");

    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();


    println!("{}",contents);
}
