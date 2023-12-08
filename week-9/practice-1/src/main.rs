use std::io::Write;

fn main() {


    let announce = "Week 9 - Rust file Input and Output\n";
    let dept = "Week 9 - Rust file Input and Output\n";

    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("Welcometo rust programming\n".as_bytes()).expect("write failed");
    file.write_all(announce.as_bytes()).expect("write failed");
    file.write_all(dept.as_bytes()).expect("write failed");


    println!("\nData written in file.");
}
