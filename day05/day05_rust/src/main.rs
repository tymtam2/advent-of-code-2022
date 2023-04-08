use std::error::Error;
use std::io::BufRead;

enum CrateMoverModel {
    CM9000,
    CM9001,
}

fn main() {
    let show_logs = true;

    match day05(show_logs, CrateMoverModel::CM9000 ) {
        Ok(x) => println!("Part 1: {}", x),
        Err(e) => println!("Error in part 1: {}", e),
    }

    match day05(show_logs, CrateMoverModel::CM9001 ) {
        Ok(x) => println!("Part 2: {}", x),
        Err(e) => println!("Error in part 2: {}", e),
    }
}

fn day05(show_logs: bool, model: CrateMoverModel) -> Result<String, Box<dyn Error>> {
    let filename = "../input05.txt";
    let file = match std::fs::File::open(filename) {
        Ok(v) => v,
        Err(error_value) => {
            return Err(Box::from(format!(
                "Could not open file '{filename}': {error_value}"
            )));
        }
    };

    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut line_num = -1;

    let mut line_stash: Vec<String> = Vec::new();

    let mut is_stage1 = true;
    for line in std::io::BufReader::new(file).lines() {
        line_num += 1;
        let line = match line {
            Ok(v) => v,
            Err(error_value) => {
                return Err(Box::from(format!("Bad line ({line_num}): {error_value}")));
            }
        };

        if is_stage1 {
            if !line.is_empty() {
                line_stash.push(line);
                continue;
            }

            if line_stash.len() < 2 {
                return Err(Box::from(format!("No enough lines with data: {line_num}")));
            }

            let len = line_stash.len();
            let row_with_numbers = &line_stash[len - 1];

            let stacks_n = match row_with_numbers.trim().split(" ").collect::<Vec<&str>>().last() {
                Some(v) => match v.parse::<i32>() {
                    Ok(v) => v,
                    Err(error_value) => {
                        return Err(Box::from(format!(
                            "Could not parse number of stacks. Line num: {line_num}, line: '{row_with_numbers}', v: '{v}', error:  {error_value}"
                        )))}
                    },                
                    None => {
                    return Err(Box::from(format!(
                        "Could not find number of stacks: {line_num}, {row_with_numbers}"
                    )));
                }
            };

            // print!("{}: ", stacks_n);

            for _ in 0..stacks_n {
                stacks.push(Vec::new());
            };

            for row in (0..len-1).rev() {
                let row = &line_stash[row];
                for column in 1..=stacks_n {
                    match row.chars().nth((column * 4 - 2 -1 ) as usize) {
                        Some(v) => {
                            if v != ' ' {
                                stacks[(column-1) as usize].push(v)
                            }
                        },
                        None => (), 
                    }; 
                }     
            }

            if show_logs {
                let tops =  stacks
                    .iter()
                    .map(|x| x.last().unwrap())
                    .collect::<String>();

                println!("{}", tops);
            }

            is_stage1 = false;
            continue;
        }

        let parts = line.split(" ").collect::<Vec<&str>>();

        if parts.len() < 6 {
            return Err(Box::from(format!("Bad line ({line_num}): {line}")));
        }

        let n_to_move = match parts[1].parse::<usize>() {
            Ok(v) => v,
            Err(_) => {
                return Err(Box::from(format!(
                    "Could not parse number of crates to move. Line num: {line_num}, line: '{line}'"
                )))
            }
        };

        let stack_from_1based = match parts[3].parse::<usize>() {
            Ok(v) => v,
            Err(_) => {
                return Err(Box::from(format!(
                    "Could not parse stack index FROM. Line num: {line_num}, line: '{line}'"
                )))
            }
        };

        let stack_to_1based = match parts[5].parse::<usize>() {
            Ok(v) => v,
            Err(_) => {
                return Err(Box::from(format!(
                    "Could not parse stack index TO. Line num: {line_num}, line: '{line}'"
                )))
            }
        };

        // if show_logs {
        //     println!(
        //         "Move {} from {} to {}",
        //         n_to_more, stack_from_1based, stack_to_1based
        //     );
        // }

        match model 
        {
            CrateMoverModel::CM9000 => for _i in 0..n_to_move {
                match stacks[stack_from_1based -1].pop()
                {
                    Some(v) => stacks[stack_to_1based - 1].push(v),
                    None => {
                        return Err(Box::from(format!(
                            "Could not pop crate from stack {stack_from_1based}. Line num: {line_num}, line: '{line}'"
                        )))
                    }
                };
                },
            CrateMoverModel::CM9001 =>
            {
                let len = stacks[stack_from_1based -1].len();

                let crates = stacks[stack_from_1based -1].drain(len-n_to_move..);

                let mut temp = Vec::new();
                temp.extend(crates);

                stacks[stack_to_1based - 1].extend(temp);
            }
        }
    }

    let mut tops = Vec::new();
    for stack in stacks {
        match stack.last() {
            Some(v) => tops.push(v.clone()),
            None => (),
        }
    }
    let answer = tops.iter()
        //.map(|&x| x.clone())
        .collect::<String>();
    Ok(answer)
}
