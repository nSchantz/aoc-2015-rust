const SECRET_KEY : &str = "yzbqklnj";

fn main() { 
    let mut hash;
    let mut count = 0;
    let mut five_match = false;
    loop {
        hash = md5::compute(format!("{}{}", SECRET_KEY, count)); 
        let hash = format!("{:x}", hash); 
        let mask = &hash[..6];

        if &mask[..5] == "00000" && !five_match {
            println!("Match-Five Zeros: {}", hash);
            println!("Minimum Number: {}", count);
            five_match = true;
        } else if mask == "000000" {
            println!("Match-Six Zeroes: {}", hash);
            println!("Minimum Number: {}", count);
            return ();
        }
        count = count + 1;
    } 
}
