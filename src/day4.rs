use crate::reader::read_txt;

const FIELDS: [&'static str; 8] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

pub fn part_1() -> std::io::Result<()> {
    let filename = "files/day4.txt";
    let lines = read_txt(filename)?;
    let contents = lines.join("\n");
    let passports = contents.split("\n\n").collect::<Vec<&str>>();

    let mut count = 0;
    let number_of_fields = FIELDS.len() - 1;
    for passport in &passports {
        let mut f_count = 0;
        for f in &FIELDS {
            if *f == "cid" {
                continue;
            }
            if passport.contains(*f) {
                // println!("true");
                f_count += 1;
            }
            if f_count == number_of_fields {
                count += 1;
            }
        }
    }
    println!("Number of valid passports: {}.", count);
    Ok(())
}
