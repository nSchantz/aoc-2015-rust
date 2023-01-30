use std::{io::Read, fs::File};
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let mut file = File::open("../input")?; 
    let mut input = String::new(); 
    file.read_to_string(&mut input)?;

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut banned_strings : HashSet<String> = HashSet::new();
    banned_strings.insert("ab".to_string());
    banned_strings.insert("cd".to_string());
    banned_strings.insert("pq".to_string());
    banned_strings.insert("xy".to_string());

    let mut num_vowels : i32;

    let mut nice_words : i32 = 0;
    let mut naughty_words : i32 = 0;

    for line in input.lines() {
        let mut vowel_check : bool = false;
        let mut double_check : bool = false;
        let mut banned_check : bool = false;
        let mut prev_char: char = 'a';
        num_vowels = 0;
        
        for char in line.char_indices() {            
            if vowels.contains(&char.1) {
                num_vowels = num_vowels + 1;
            } 

            if char.0 != 0 {
                if prev_char == char.1 {
                    double_check = true;
                }

                let mut two_char = String::from(prev_char);
                two_char.push(char.1);
                if banned_strings.contains(&two_char) {
                    banned_check = true;
                } 
            }
      
            prev_char = char.1            
        }

        if num_vowels >= 3 {
            vowel_check = true;
        }

        if vowel_check && double_check && !banned_check {
            nice_words = nice_words + 1;
        } else {
            naughty_words = nice_words + 1;
        }
    }

    println!("Nice Words: {}", nice_words);
    println!("Naughty Words: {}", naughty_words);

    Ok(())
}
