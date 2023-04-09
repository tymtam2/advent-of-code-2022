use std::error::Error;
use std::io::BufRead;

fn main() {
    let show_logs = false;

    match day07(show_logs, "../input07_a_example.txt") {
        Ok(x) => println!("Part 1: {}", x),
        Err(e) => println!("Error in part 1: {}", e),
    }

    match day07(show_logs, "../input07.txt") {
        Ok(x) => println!("Part 1: {}", x),
        Err(e) => println!("Error in part 1: {}", e),
    }
}

#[derive(Debug)]
struct Item {
    id: usize,
    name: String,
    size: usize,      // 0 for directories
    parent_id: usize, // for root, this is 0
}

fn day07(_show_logs: bool, filename: &str) -> Result<i32, Box<dyn Error>> {
    let file = match std::fs::File::open(filename) {
        Ok(v) => v,
        Err(error_value) => {
            return Err(Box::from(format!(
                "Could not open file '{filename}': {error_value}"
            )));
        }
    };

    let mut line_num = 0;
    let mut highest_id = 0;

    let mut drive = Vec::new();
    drive.push(Item {
        id: 0,
        name: String::from("/"),
        size: 0,
        parent_id: 0,
    });
    let mut current_i = 0;
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

        if line_num == 0 && line != "$ cd /" {
            return Err(Box::from(format!(
                "First line should be '$ cd /', but it is '{line}'"
            )));
        }

        if line.starts_with("$ ") {
            if line == "$ ls" {
                println!("ls on '{}'", drive[current_i].name);
                continue;
            }

            if line == "$ cd .." {
                println!("ls on '{}'", drive[current_i].name);
                current_i = find_parent(&drive, current_i);
                continue;
            }

            // $ cd a
            let dir_name = &line[5..];

            current_i = find_child_dir(&drive, dir_name, current_i);
            continue;
        } else {
            if line.starts_with("dir ") {
                // dir e
                let dir_name = &line[4..];
                println!("dir {}", dir_name);

                highest_id += 1;
                drive.push(Item {
                    id: highest_id,
                    name: String::from(dir_name),
                    size: 0,
                    parent_id: drive[current_i].id,
                });
                continue;
            }

            // it's a file
            // 29116 f
            let parts = line.split(" ").collect::<Vec<&str>>();
            let file_size = parts[0]
                .parse::<usize>()
                .unwrap_or_else(|e| panic!("Bad file size:  ({line_num})  '{line}': {e}"));

            let file_name = parts[1];

            highest_id += 1;
            drive.push(Item {
                id: highest_id,
                name: String::from(file_name),
                size: file_size,
                parent_id: current_i,
            });
        }
    }

    print!("drive: {:?}\n", drive);

    let mut size_under_100k = 0;
    for dir in get_all_directries(&drive) {
        let size = get_size(&drive, dir.id);
        println!("dir: {} {}", dir.name, size);
        if size < 100000 {
            size_under_100k += size;
        }
    }

    return Ok(size_under_100k as i32);
}

fn find_parent(drive: &Vec<Item>, current_i: usize) -> usize {
    let me = drive
        .iter()
        .position(|x| x.id == drive[current_i].id && x.size == 0);

    if me == None {
        panic!(
            "Could not find directory '{}' in '{}'",
            current_i, drive[current_i].name
        );
    }

    let me = &drive[me.unwrap()];
    let current_parent_id = me.parent_id;

    let i = drive
        .iter()
        .position(|x| x.size == 0 && x.id == current_parent_id);

    if i == None {
        panic!("Could not find parent of '{}'", me.name);
    }

    return i.unwrap();
}

fn find_child_dir(drive: &Vec<Item>, dir_name: &str, current_i: usize) -> usize {
    let i = drive
        .iter()
        .position(|x| x.parent_id == drive[current_i].id && x.size == 0 && x.name == dir_name);

    if i == None {
        panic!(
            "Could not find directory '{}' in '{}'",
            dir_name, drive[current_i].name
        );
    }

    return i.unwrap();
}

fn get_all_directries(drive: &Vec<Item>) -> Vec<&Item> {
    let mut dirs = Vec::new();

    for item in drive {
        if item.size == 0 {
            dirs.push(item.clone());
        }
    }
    return dirs;
}

fn get_size(drive: &Vec<Item>, current_i: usize) -> usize {
    let mut size = 0;

    let current_id = drive[current_i].id;

    if (current_id == 0) {
        for item in drive {
            if (item.size == 0) {
            } else {
                size += item.size;
            }
        }
        return size;
    }

    for item in drive {
        if item.parent_id == current_id {
            if (item.size == 0) {
                size += get_size(drive, item.id);
            } else {
                size += item.size;
            }
        }
    }
    return size;
}
