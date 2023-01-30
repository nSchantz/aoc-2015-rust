use std::{io::Read, fs::File};
use std::collections::HashMap;

struct Point {
    x : i32,
    y : i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn mv_up(&mut self) {
        self.y = self.y + 1;
    }

    fn mv_left(&mut self) {
        self.x = self.x - 1;
    }

    fn mv_right(&mut self) {
        self.x = self.x + 1;
    }

    fn mv_down(&mut self) {
        self.y = self.y - 1;
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("../input")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let mut map : HashMap<(i32, i32), bool> = HashMap::new(); 
    let mut santa_coords : Point = Point::new(0, 0);
    let mut robo_coords : Point = Point::new(0, 0);
    let mut first_count : i32 = 0;

    for (count, dir) in input.chars().enumerate() {

        if count % 2 == 0 {
            mov_dir(&mut santa_coords, &dir);
            //print!("Santa: {}, {}", santa_coords.x, santa_coords.y);
            first_count = first_count + check_visit(&santa_coords, &mut map);
        } else {
            mov_dir(&mut robo_coords, &dir);
            //print!("Robo: {}, {}", robo_coords.x, robo_coords.y);
            first_count = first_count + check_visit(&robo_coords, &mut map);
        }
        
    }
    
    // Adding two for the starting location.
    println!("Houses visited at least once: {}", first_count);

    Ok(())
}

fn mov_dir(coords: &mut Point, dir: &char) {
    match dir {
        '^' => coords.mv_up(),
        '<' => coords.mv_left(),
        '>' => coords.mv_right(),
        'v' => coords.mv_down(),
        _ => ()
     };
}

fn check_visit(coords: &Point, map: &mut HashMap<(i32, i32), bool>) -> i32 {
    match map.get(&(coords.x, coords.y)) {
        Some(true) => {
            //println!("");
            0
        },
        None => {
            //println!(" New");
            map.insert((coords.x, coords.y), true);
            1
        },
        _ => 0
    }
}
