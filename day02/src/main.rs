use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path};

fn main() {
    println!("Hello, world!");
    //Get ranges
    let ranges: Vec<(i64, i64)> = get_ranges("src/input.txt").expect("Could not read file");
    for (a, b) in &ranges {
        println!("from {} to {}", a, b);
    }
    //Get all ids
    println!("Saving ALL ids in delicous memory mmmmmmmmmmm..");
    let ids: Vec<String> = get_ids(&ranges);
    println!("wow, now im stuffed with {} ids!", ids.len());

    //count amount of fake ids:
    let fake_total = get_fake_ids_total(&ids);
    println!("The total amount of fake is {}", fake_total);
}

fn get_ranges<P>(filepath: P) -> io::Result<Vec<(i64, i64)>>
where
    P: AsRef<Path>, {
    let file: File = File::open(filepath)?;
    let reader = BufReader::new(file);
    let input = reader.lines();
    let mut pairs: Vec<(i64, i64)> = Vec::new();

    for line in input {
        let line = line?;
        for range in line.split(',') { //split into ranges by , and loop over
            let parts: Vec<&str> = range.split('-').collect(); //split each start and end into a parts vector by -
            if parts.len() == 2 {
                let start: i64 = parts[0].parse().unwrap();
                let end: i64 = parts[1].parse().unwrap();
                pairs.push((start, end)); //push the pair to pairs vector after unwrap
            }
        }
    }
    return Ok(pairs);
}

fn get_ids(ranges: &Vec<(i64, i64)>) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    for (a, b) in ranges {
        ids.extend((*a..=*b).map(|x| x.to_string())); //mapping is magic
    }
    return ids;
}

fn get_fake_ids_total(ids: &Vec<String>) -> i64 {
    let mut total: i64 = 0;
    for id in ids {
        if (id.len() % 2) == 0 { //skip iteration if not even length
            let (a, b) = id.split_at(id.len()/2);
            if a == b {
                total += id.parse::<i64>().unwrap();
            }
        }
    }
    return total; 
}

fn get_fake_ids_total_too(ids: &Vec<String>) -> i64 {
    
}