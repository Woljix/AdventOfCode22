use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let mut elves: Vec<u32> = Vec::new();

    if let Ok(lines) = read_lines("input") {
        let mut calorie_count: u32 = 0;

        for line in lines {
            // If line is whitespace add the calorie count to the list and reset.
            if let Ok(_number) = line {
                if _number.is_empty() {
                    println!("Elf number '{}' added to list with a total of '{}' calories.", elves.len() + 1, calorie_count);
                    elves.push(calorie_count);
                    calorie_count = 0;
                }
                else {
                    if let Ok(calorie) = _number.parse::<u32>() {
                        println!("Adding '{}' calories to elf number: '{}'.", calorie, elves.len() + 1);
                        calorie_count += calorie;
                    }
                }
            }
        }
    }

    println!("The highest calorie count of any one elf is {}", elves.iter().max().unwrap());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
