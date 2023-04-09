use std::error::Error;
use std::fs;
use std::io::BufRead;

fn main() {
    let show_logs = false;

    match day06(show_logs) {
        Ok(x) => println!("Part 1: {}", x),
        Err(e) => println!("Error in part 1: {}", e),
    }

    let filename = "../input06_b_examples.txt";
    //open file
    let file = std::fs::File::open(filename)
        .unwrap_or_else(|e| panic!("Could not open file '{filename}': {e}"));

    let mut line_num = 0;
    for line in std::io::BufReader::new(file).lines() {
        line_num += 1;
        let line = line.unwrap_or_else(|e| panic!("Bad line ({line_num}): {e}")); 
        match day06_part2(false, line) {
            Ok(x) => println!("Part 2: {}", x),
            Err(e) => println!("Error in part 2: {}", e),
        }
    }
    let filename = "../input06.txt";
    let real = fs::read_to_string(filename)
        .expect(format!("Something went wrong reading the file {filename}").as_str());

    match day06_part2(false, real) {
        Ok(x) => println!("Part 2: {}", x),
        Err(e) => println!("Error in part 2: {}", e),
    }
}

fn day06(_show_logs: bool) -> Result<i32, Box<dyn Error>> {
    let filename = "../input06.txt";
    let s = fs::read_to_string(filename)
        .expect(format!("Something went wrong reading the file {filename}").as_str());

    //let vec = Vec::new();

    if _show_logs {
        print!("{}\n", s);
    }
    for i in 4..=s.len() {
        let word = s.get(i - 4..i).unwrap();

        if _show_logs {
            print!("{}{}", " ".repeat(i - 4), word);
        }

        let word = word.as_bytes();

        if word[0] == word[1]
            || word[0] == word[2]
            || word[0] == word[3]
            || word[1] == word[2]
            || word[1] == word[3]
            || word[2] == word[3]
        {
            if _show_logs {
                print!(" dup: {i} \n");
            }
            continue;
        }
        return Ok(i as i32);
    }
    Ok(0)
}

fn day06_part2(show_logs: bool, s1: String) -> Result<i32, Box<dyn Error>> {
    if show_logs {
        print!("{}\n", s1);
    }

    let mut i = 0;

    let s = s1.as_bytes();

    let n = s.len();

    if n < 14 {
        return Err(Box::from(format!("String '{}' is too short: {} < 14", s1,  n)));
    }

    'main: while i <= n - 14 {
        for ci in (0..=13).rev() {
            let c1 = s[i + ci];
            for cj in ci + 1..=13 {
                let c2 = s[i + cj];
                //print!("{}{} ", c1 as char, c2 as char);
                if c1 == c2 {
                    if show_logs {
                        let dup_word = s1.get(i..=i + cj).unwrap();
                        print!("{}{} ({})\n", " ".repeat(i), dup_word, c1 as char);
                    }
                    i = i + (ci + 1);

                    continue 'main;
                }
            }
        }

        if show_logs {
            let word = s.get(i..i + 14).unwrap();
            print!(
                "{}{}\n",
                " ".repeat(i),
                word.iter().map(|x| *x as char).collect::<String>()
            );
            print!("{}^\n", " ".repeat(i + 14));
            print!(
                "{}\n",
                (0..i + 14)
                    .map(|x| (((x % 10) + 48) as u8) as char)
                    .collect::<String>()
            );
        }
        return Ok(i as i32 + 14);
    }

    Ok(0)
}
