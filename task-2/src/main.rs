use std;

fn main() -> Result<(), std::io::Error> {
    let file: &str = include_str!("../file_example.txt");
    let lines: Vec<&str> = file.split(",").collect();
    println!("{:?}", lines);

    Ok(())
}
