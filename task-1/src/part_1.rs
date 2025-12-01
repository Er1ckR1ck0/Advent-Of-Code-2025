use std;
use std::fs::File;
use std::io::{ BufReader, BufRead};

fn count_index(direction: &str, integer: i32, steps: i32) -> i32 {
    let mut steps = steps % 100;

    let result = if direction == "L" {
        integer - steps
    }
    else {
        integer + steps
    };
    result.rem_euclid(100)
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("lock.txt")?;

    let reader = BufReader::new(file);

    let mut current_item: i32 = 50;
    let mut password = 0;
    for line in reader.lines() {
        let line = line?;
        let direction: &str = line[0..1].into();
        let steps: i32 = line[1..].parse().unwrap_or(0);
        current_item = count_index(direction, current_item, steps);
        if current_item == 0 || current_item == 100 {
            password += 1;
        }
        
    }
    println!("Password attempts: {}", password);

    Ok(())
}