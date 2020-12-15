// --- Day 2: Password Philosophy ---

// Your flight departs in a few days from the coastal airport; the easiest way down to the coast from here is via toboggan.

// The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong with our computers; we can't log in!" You ask if you can take a look.

// Their password database seems to be a little corrupted: some of the passwords wouldn't have been allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.

// To try to debug the problem, they have created a list (your puzzle input) of passwords (according to the corrupted database) and the corporate policy when that password was set.

// For example, suppose you have the following list:

// 1-3 a: abcde
// 1-3 b: cdefg
// 2-9 c: ccccccccc

// Each line gives the password policy and then the password. The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid. For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.

// In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no instances of b, but needs at least 1. The first and third passwords are valid: they contain one a or nine c, both within the limits of their respective policies.

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn password_philosophy() -> std::io::Result<()> {
    let filename = "files/passwords.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut count = 0;
    for line in reader.lines() {
        let data = line.unwrap();
        let split_data: Vec<&str> = data.split(' ').collect();
        if split_data.len() == 1 {
            break;
        }

        let range: Vec<&str> = split_data[0].split('-').collect();
        let lower_bound = range[0].parse::<i32>().unwrap() - 1;
        let upper_bound = range[1].parse::<i32>().unwrap() + 1;
        // ----- DEBUGGING -----
        // println!("{}, {}", lower_bound, upper_bound);
        // let target_char: Vec<&str> = split_data[1].split(':').collect();
        // let mut target_char = split_data[1].chars();
        // target_char.next();
        // println!("{:?}", target_char.as_str());
        // -----    END    -----
        let target_char = split_data[1].chars().next().unwrap();
        // println!("{}", target_char);

        let password = split_data[2];
        // println!("{}", password);

        let mut char_count = 0;
        for c in password.chars() {
            if c == target_char {
                char_count += 1;
            }
        }

        if char_count > lower_bound && char_count < upper_bound {
            count += 1;
        }
    }

    println!("{}", count); // Print the number of valid passwords.
    Ok(())
}
