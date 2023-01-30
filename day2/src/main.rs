use std::{fs::File, io::Read};

fn main() -> std::io::Result<()> {
    let mut file = File::open("../input")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let mut wrapping_paper = 0;   
    let mut ribbon = 0;
    let mut dims : Vec<i32>;
    let mut max;
    let mut side_one : i32;
    let mut side_two : i32;
    let mut side_three : i32;
    let mut volume : i32;
    let mut filtered_max : bool = false;

    for line in input.lines() {
        // Split by delimiter 'x' into an iterator. Use map to chance all values in iterator from str to i32. Use collect to turn iterator into collection
        dims = line.split('x').map(|x| x.parse::<i32>().unwrap()).collect();
        max = dims.iter().max().unwrap();
        let perimeter : Vec<&i32> = dims.iter().filter(|x| 
            if **x == *max && !filtered_max {
                filtered_max = true;
                false
            } else {
                true
            }
        ).collect();

        filtered_max = false;

        side_one = dims[0] * dims[1];
        side_two = dims[1] * dims[2];
        side_three = dims[0] * dims[2]; 
        volume = dims[0] * dims[1] * dims[2];

        let smallest_side = vec![side_one, side_two, side_three];
        let smallest_side = smallest_side.iter().min().unwrap();

        //println!("{}, {}, {}, {}", side_one, side_two, side_three, smallest_side);
        //println!("{}, {}, {}", volume, perimeter[0], perimeter[1]);

        wrapping_paper = wrapping_paper + 2 * side_one + 2 * side_two + 2 * side_three + smallest_side;
        ribbon = ribbon + volume + 2 * perimeter[0] + 2 * perimeter[1];
    }

    println!("Needed square feet of wrapping paper: {}", wrapping_paper);
    println!("Total feet of ribbon needed: {}", ribbon);
    
    Ok(())
}
