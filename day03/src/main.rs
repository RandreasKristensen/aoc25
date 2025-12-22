use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path};

fn main() {
    //get input file
    let path = "src/input.txt";
    let banks= read_file(&path).expect("Could not read file");
    // Create list of joltages
    let joltages: Vec<i64> = get_joltages(&banks);
    //calculate the sum and result of task 1
    let total_joltage: i64 = joltages.into_iter().sum();
    println!("the total joltage is {}", total_joltage);
}

fn get_joltages(banks: &Vec<String>) -> Vec<i64> {
    let mut joltages: Vec<i64> = Vec::new();
    for bank in banks {
        joltages.push(get_joltage(&bank))
    }
    return joltages;
}

fn get_joltage(bank: &String) -> i64 {
    let mut first_num : i64 = 0;
    let mut second_num : i64 = 0;
    let chars: Vec<char> = bank.chars().collect();
    for (i, char) in chars.iter().enumerate() {
        if char.is_numeric() {
            let digit = char.to_digit(10).unwrap() as i64;
            if digit > first_num {
                if i == chars.len() - 1 {  // is last digit
                    second_num = digit;
                } else {
                    first_num = digit;
                    second_num = 0;
                }
            }
            else if digit > second_num {
                second_num = digit;
            }
        }
    }
    let joltage : i64 = ( first_num * 10 ) + second_num;
    println!("joltage for bank {} is {}", bank, joltage);
    return joltage;
}


fn read_file<P>(filepath: P) -> io::Result<Vec<String>>
where P: AsRef<Path>, {
    let file = File::open(filepath)?;
    let buf = io::BufReader::new(file);
    buf.lines().collect()
}