use std::{io::Read, fs::File, ops::Index};

fn main() -> std::io::Result<()> {
    let mut file = File::open("../input")?; 
    let mut input = String::new(); 
    file.read_to_string(&mut input)?;
 
    let mut num_vowels : i32;

    for line in input.lines() {
        num_vowels = 0;

        for char in (0..line.len() - 1) {
            if line.index(0) == 'a' {
                num_vowels = num_vowels + 1;
            } 
        }
    }

    Ok(())
}
