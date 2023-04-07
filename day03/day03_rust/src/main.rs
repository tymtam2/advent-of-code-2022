use std::error::Error;
use std::io::BufRead;

fn main() {

    let show_logs = false;
    
    match day03_part1() {
        Ok(x) => println!("Part 1: {}", x),
        Err(e) => println!("Error in part 1: {}", e),
    }

    match day03_part2(show_logs) {
        Ok(x) => println!("Part 2: {}", x),
        Err(e) => println!("Error in part 2: {}", e),
    }
}

fn day03_part1() -> Result<i32, Box<dyn Error>> {
    let filename = "../input03.txt";
    let file = match std::fs::File::open(filename) {
        Ok(v) => v,
        Err(error_value) => {
            return Err(Box::from(format!(
                "Could not open file '{filename}': {error_value}"
            )));
        }
    };

    let mut x: i32 = 0;
    let mut line_num = 0;
    for line in std::io::BufReader::new(file).lines() {
        line_num += 1;
        let line = match line {
            Ok(v) => v,
            Err(error_value) => {
                return Err(Box::from(format!("Bad line ({line_num}): {error_value}")));
            }
        };

        if line.is_empty() {
            continue;
        }

        let len = line.len();

        if line.len() % 2 == 1 {
            return Err(Box::from(format!(
                "Line {line} has an odd number of characters: {len}"
            )));
        }

        let half_len = len / 2;
        // let first_half = line[..half_len].chars();
        // let last_half = line[half_len..].chars();

        // assert_eq!(first_half.count(), last_half.count());

        //print!("{} -> {} {} ",line,  &line[..half_len], &line[half_len..]);

        let chars = line.as_bytes();

        let mut c = 0;
        // not validating if there are more than one duplicates
        for i1 in 0..half_len {
            for i2 in half_len..len {
                //print!("{}_{} ", chars[i1] as char, chars[i2] as char);

                if chars[i1] == chars[i2] {
                    c = chars[i1];
                    break;
                }
            }
            if c != 0 {
                break;
            }
        }

        if c == 0 {
            return Err(Box::from(format!("No duplicates in line '{line}'")));
        }

        let p = match get_priority(c) {
            Some(v) => v,
            None => {
                return Err(Box::from(format!(
                    "Unknown character '{c}' in line '{line}'"
                )))
            }
        };

        //print!(" dup: {}({}) \n", c as char, p);

        x += p as i32;
    }
    Ok(x)
}

fn get_priority(c: u8) -> Option<u8> {
    return match c {
        65..=90 => Some(c - 65 + 27),
        97..=122 => Some(c - 97 + 1),
        _ => None,
    };
}

fn day03_part2(logs: bool) -> Result<i32, Box<dyn Error>> {
    let filename = "../input03.txt";
    let file = match std::fs::File::open(filename) {
        Ok(v) => v,
        Err(error_value) => {
            return Err(Box::from(format!(
                "Could not open file '{filename}': {error_value}"
            )));
        }
    };

    let mut x: i32 = 0;
    let mut line_num = 0;

    let mut v: Vec<String> = Vec::new();
    for line in std::io::BufReader::new(file).lines() {
        line_num += 1;
        let line = match line {
            Ok(v) => v,
            Err(error_value) => {
                return Err(Box::from(format!("Bad line ({line_num}): {error_value}")));
            }
        };

        if line.is_empty() {
            continue;
        }

        let len = line.len();

        if line.len() % 2 == 1 {
            return Err(Box::from(format!(
                "Line {line} has an odd number of characters: {len}"
            )));
        }

        v.push(line);

        let mut c: char = ' ';
        if v.len() < 3 {
            continue;
        }

        for c0 in v[0].chars() {
            for c1 in v[1].chars() {
                if c0 == c1 {
                    for c2 in v[2].chars() {
                        if c0 == c2 {
                            c = c0;
                        }
                    }
                }
            }
        }

        if c == ' ' {
            return Err(Box::from(format!(
                "No coomon character in lines {}\n{}\n{}",
                v[0], v[1], v[2]
            )));
        }

        let p = match get_priority(c as u8) {
            Some(v) => v,
            None => {
                return Err(Box::from(format!(
                    "Unknown character '{c}' in batch staring with '{}'",
                    v[0]
                )))
            }
        };
        if logs {
            let c2 = format!("_{c}_");
            print!(
                "{} {} {} -> {}({}) \n",
                v[0].replace(c, &c2),
                v[1].replace(c, &c2),
                v[2].replace(c, &c2),
                c,
                p
            );
        }

        x += p as i32;

        v.clear();
    }

    Ok(x)
}
