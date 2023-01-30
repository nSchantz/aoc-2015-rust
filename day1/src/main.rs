use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = File::open("../input")?; // Get file handle to input
    let mut input = String::new(); // Create a empty string  for input
    file.read_to_string(&mut input)?; // Read input into string

    let mut floor: i32 = 0; 
    let mut base_found: bool = false;
    let mut base_pos: usize = 0;

    for (i, dir) in input.chars().enumerate() {
        match dir {
            '(' => floor = floor + 1,
            ')' => floor = floor - 1,
            ' ' => {},
            _ => {},
        }
        if !base_found && floor == -1 {
            base_found = true;
            base_pos = i + 1; 
        }
    }

    println!("Floor: {}", floor);
    println!("Position that enters basement: {}", base_pos);
    
    Ok(()) 
}
