use std::io::{ BufRead };

fn main() {
    let filename = "input01.txt";
    let file = std::fs::File::open(filename).unwrap_or_else(|_|panic!("File '{}' not found", filename));


    let mut vec = Vec::new();
    vec.push(0);

    for line in std::io::BufReader::new(file).lines() {
        let line = line.unwrap();
        
        if line.is_empty() {
            vec.push(0);
        }
        else {
            let i = vec.len() - 1;
            vec[i] += line.parse::<i32>().unwrap();
        }
    }

    vec.sort();
    let i = vec.len() - 1;
    println!("{}", vec[i]);

    // part 2

    vec.reverse();

    if vec.len() < 3
    {
        panic!("Not enough data");
    }
    
    println!("{}", vec[0]+ vec[1]+vec[2]);

}
