use std::error::Error;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    match day02_part1() {
        Ok(()) => (),
        Err(e) => println!("Error in part 1: {}", e),
    }

    match day02_part2() {
        Ok(()) => (),
        Err(e) => println!("Error in part 2: {}", e),
    }
}

fn day02_part1() -> Result<(), Box<dyn Error>> {

    let scores = HashMap::from([
        ( "A X", 1 + 3 ),
        ( "B X", 1 + 0 ),
        ( "C X", 1 + 6 ),
        ( "A Y", 2 + 6),
        ( "B Y", 2 + 3),
        ( "C Y", 2 + 0),
        ( "A Z", 3 + 0),
        ( "B Z", 3 + 6),
        ( "C Z", 3 + 3)
    ]);

    let n = get_n(scores)?;

    println!("n: {}", n);

    Ok(())
}

fn day02_part2() -> Result<(), Box<dyn Error>> {

    let scores = HashMap::from([
        ( "A X", 3 + 0),
        ( "B X", 1 + 0),
        ( "C X", 2 + 0),
        ( "A Y", 1 + 3),
        ( "B Y", 2 + 3),
        ( "C Y", 3 + 3),
        ( "A Z", 2 + 6),
        ( "B Z", 3 + 6),
        ( "C Z", 1 + 6),
    ]);

    let n = get_n(scores)?;

    println!("n: {}", n);

    Ok(())
}

fn get_n(scores: HashMap<&str, i32>) -> Result<i32, Box<dyn Error>> {

    let filename = "../input02.txt";
    let file = std::fs::File::open(filename)?;

    let mut n: i32 = 0;
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
        } else {
            match scores.get(&*line) {
                Some(&number) => n += number,
                _ => return Err(Box::from(format!("Unknown instruction '{line}' in line {line_num}")))
            }
        }
    }

    Ok(n)
}
