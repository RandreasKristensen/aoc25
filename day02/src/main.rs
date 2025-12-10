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

    //count total amount of fake:
    let fake_total = get_fake_ids_total(&ids);
    println!("The total amount of fake is {}", fake_total);

    //count amount of fake for part two:
    let fake_total_too: i64 = get_fake_ids_total_too(&ids);
    println!("the total amount of fake is {}, too", fake_total_too);
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
    let mut total: i64 = 0;
    'outer: for id in ids {
        let id_length = id.len();
        // Try each possible chunk size from 1 to half the ID length
        for chunk_size in 1..=(id_length/2) {
            if id_length % chunk_size == 0 {
                let num_chunks = id_length / chunk_size;
                // Check if all chunks are identical
                let first_chunk = &id[0..chunk_size];
                let mut all_equal = true;
                for chunk_index in 1..num_chunks {
                    let start = chunk_index * chunk_size;
                    let end = start + chunk_size;
                    if &id[start..end] != first_chunk {
                        all_equal = false;
                        break;
                    }
                }
                if all_equal {
                    total += id.parse::<i64>().unwrap();
                    continue 'outer;
                }
            }
        }
    }
    total
}