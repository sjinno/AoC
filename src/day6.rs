use crate::reader::{clean_input, read_txt};
use std::collections::HashSet;

#[allow(unused_macros)]
// Debug format.
macro_rules! printd {
    ($v:expr) => {
        println!("{:?}", $v);
    };
}

#[allow(unused_macros)]
// Format.
macro_rules! printf {
    ($v:expr) => {
        println!("{}", $v);
    };
}

pub fn part1() -> std::io::Result<()> {
    let filename = "files/day6.txt";
    // let filename = "files/day6_example.txt";
    let contents = read_txt(filename)?;
    let cleaned_input = clean_input(contents);
    // printf!(cleaned_input);
    let groups: Vec<&str> = cleaned_input.split("\n\n").collect();
    // printd!(groups);
    // printf!(groups.len());
    // let mut g_count = 0;
    let mut num_of_questions = 0;
    let mut hs: HashSet<char> = HashSet::new();
    for g in groups {
        let remove_ln = str::replace(g, "\n", "");
        for c in remove_ln.chars() {
            if !hs.contains(&c) {
                hs.insert(c);
            }
        }
        num_of_questions += hs.len();
        hs.clear();
    }
    printf!(num_of_questions);
    // printf!(g_count);
    Ok(())
}
