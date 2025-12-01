use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn count_index(direction: &str, current_pos: i32, steps: i32) -> (i32, i32) {
    let mut crossings = steps / 100;
    let remainder = steps % 100;

    let new_pos = if direction == "R" {
        if current_pos + remainder >= 100 {
            crossings += 1;
        }
        (current_pos + remainder) % 100
    } else {
        if current_pos != 0 && current_pos - remainder <= 0 {
            crossings += 1;
        }
        (current_pos - remainder).rem_euclid(100)
    };

    (crossings, new_pos)
}

fn main() -> Result<(), io::Error> {
    let file = File::open("lock.txt")?;
    let reader = BufReader::new(file);

    let mut current_item = 50;
    let mut password = 0;

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() { continue; }

        let direction = &line[0..1];
        let steps: i32 = line[1..].parse().unwrap_or(0);

        let (crossings, result) = count_index(direction, current_item, steps);

        current_item = result;
        password += crossings;
    }

    println!("Password attempts: {}", password);
    Ok(())
}