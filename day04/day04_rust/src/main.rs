use std::error::Error;
use std::io::BufRead;

fn main() {
    let show_logs = true;

    match day04_part1(show_logs) {
        Ok(x) => println!("Part 1: {}", x),
        Err(e) => println!("Error in part 1: {}", e),
    }

    match day04_part2(show_logs) {
        Ok(x) => println!("Part 2: {}", x),
        Err(e) => println!("Error in part 2: {}", e),
    }
}

fn day04_part2(show_logs: bool) -> Result<i32, Box<dyn Error>> {
    let filename = "../input04.txt";
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

        let line = line.trim();

        let two = line.split(",").collect::<Vec<&str>>();

        if two.len() != 2 {
            return Err(Box::from(format!(
                "Line {line_num} does not have two values: {line}"
            )));
        }

        let a = two[0].split("-").collect::<Vec<&str>>();

        if a.len() != 2 {
            return Err(Box::from(format!(
                "Line {line_num} does not have two values: {}, {}",
                line, two[0]
            )));
        }

        let b = two[1].split("-").collect::<Vec<&str>>();

        if b.len() != 2 {
            return Err(Box::from(format!(
                "Line {line_num} does not have two values: {}, {}",
                line, two[1]
            )));
        }

        let a1 = a[0].parse::<i32>()?;
        let a2: i32 = a[1].parse()?;
        let b1 = b[0].parse::<i32>()?;
        let b2: i32 = b[1].parse()?;


        // 7-9, 9-78
        let overlaps = (a1 >= b1 && a1 <= b2) || (a2 >= b1 && a2 <= b2)
                || (b1 >= a1 && b1 <= a2) || (b2 >= a1 && b2 <= a2);

        if show_logs {
            println!("{}: {} {}", line_num, line,  if overlaps { '+' } else { '-' });
        }
        if overlaps {
            x += 1;
        }
    }
    Ok(x)
}

fn day04_part1(show_logs: bool) -> Result<i32, Box<dyn Error>> {
    let filename = "../input04.txt";
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

        let line = line.trim();

        let two = line.split(",").collect::<Vec<&str>>();

        if two.len() != 2 {
            return Err(Box::from(format!(
                "Line {line_num} does not have two values: {line}"
            )));
        }

        let a = two[0].split("-").collect::<Vec<&str>>();

        if a.len() != 2 {
            return Err(Box::from(format!(
                "Line {line_num} does not have two values: {}, {}",
                line, two[0]
            )));
        }

        let b = two[1].split("-").collect::<Vec<&str>>();

        if b.len() != 2 {
            return Err(Box::from(format!(
                "Line {line_num} does not have two values: {}, {}",
                line, two[1]
            )));
        }

        let a1 = a[0].parse::<i32>()?;
        let a2: i32 = a[1].parse()?;
        let b1 = b[0].parse::<i32>()?;
        let b2: i32 = b[1].parse()?;

        if a1 >= b1 && a2 <= b2 || b1 >= a1 && b2 <= a2 {
            if show_logs {
                println!("{}: {}", line_num, line);
            }
            x += 1;
        }
    }
    Ok(x)
}
