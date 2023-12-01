use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String>{
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
} 

fn main()
{
    let lines = read_lines("src/input/day01/actual.txt");

    let mut calib: u32 = 0;

    for line in lines {
        let numbers: Vec<u32> = line.chars().flat_map(|ch| ch.to_digit(10)).collect(); 

        // very safe unwrapping
        calib += numbers.first().unwrap() * 10 + numbers.last().unwrap() * 1;
    }
    println!("Part 1 - Calibration values = {}", calib);
}
