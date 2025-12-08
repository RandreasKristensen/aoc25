use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    //load in input from input.txt
    let lines = read_lines("src/input.txt").expect("Could not read file");

    //variables: current dial position, amount of zeros
    let mut dial_position : i32 = 50;
    let mut zero_count : i32 = 0;
    let mut hit_zero_times : i32 = 0; 


    for line in &lines {
        //process the line, add or subtract
        let (is_positive, num) = read_line(line);
        if is_positive {
            (dial_position, hit_zero_times) = add(dial_position, num);
        } else {
            (dial_position, hit_zero_times) = subtract(dial_position, num);
        }
        println!("The line is rotated {} to point at at {}, and zero was passed {} times", line, dial_position, hit_zero_times);
        zero_count += hit_zero_times;
    }

    println!("The dial hit 0 a total of {} times", zero_count);
}


fn read_line(line: &str) -> (bool, i32) {
    let is_positive: bool = line.contains("R");
    let num: i32 = line[1..].parse().expect("Not a number");
    return (is_positive, num);
}

fn add(pos: i32, x: i32) -> (i32, i32) {
    //define variables
    let mut current_pos: i32 = pos;
    let mut remaining_spin: i32 = x;
    let mut ticks: i32 = 0;
    //spin the lock and count the ticks
    while remaining_spin >= 0 {
        //calculate the amount "up" to 0 
        let amount_to_tick: i32 =  100 - current_pos;
        //if we dont have enough remaining spin to tick, update the current position and break out of the loop
        if remaining_spin < amount_to_tick {
            current_pos += remaining_spin;
            break;
        }
        //if we have enough to tick, subtract the amount to tick from the remaining spin, count the tick, and update the position to 0
        remaining_spin -= amount_to_tick;
        ticks += 1;
        current_pos = 0;
    }
    //return the position and the ticks.
    return (current_pos, ticks);
} 

fn subtract(pos: i32, x: i32) -> (i32, i32) {
    //define variables
    let mut current_pos: i32 = pos;
    let mut remaining_spin: i32 = x;
    let mut ticks: i32 = 0;
    //spin the lock and count the ticks
    while remaining_spin > 0 {
        //calculate the amount "down" to 0. its equal to the position - except for when its 0, then its 100 for a full rotation
        let amount_to_tick: i32 = if current_pos == 0 { 100 } else { current_pos };
        //If we dont have enough remaining to tick, update the current position and break out of the loop
        if remaining_spin < amount_to_tick {
            if current_pos == 0 {
                current_pos = 100 - remaining_spin;
            } else {
                current_pos -= remaining_spin;
            }
            break;
        }
        //if we have enough to tick, subtract the amount to tick from the remaining spin, count the tick, and update the position to 0
        remaining_spin -= amount_to_tick;
        ticks += 1;
        current_pos = 0;
    }
    //return the position and the ticks.
    return (current_pos, ticks);
}


fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);
    buf.lines().collect()
}