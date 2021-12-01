use std::collections::HashMap;
use std::io::BufRead;

use crate::solution::Solution;

struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

impl Passport {

    fn from_fields(mut fields: HashMap<String, String>) -> Option<Self> {
        if !Passport::contains_required_fields(&fields) {
            None
        } else {
            let passport = Passport {
                byr: fields.remove("byr").unwrap(),
                iyr: fields.remove("iyr").unwrap(),
                eyr: fields.remove("eyr").unwrap(),
                hgt: fields.remove("hgt").unwrap(),
                hcl: fields.remove("hcl").unwrap(),
                ecl: fields.remove("ecl").unwrap(),
                pid: fields.remove("pid").unwrap(),
                cid: fields.remove("cid"),
            };
            Some(passport)
        }
    }

    fn contains_required_fields(fields: &HashMap<String, String>) -> bool {
        fields.contains_key("byr") &&
        fields.contains_key("iyr") &&
        fields.contains_key("eyr") &&
        fields.contains_key("hgt") &&
        fields.contains_key("hcl") &&
        fields.contains_key("ecl") &&
        fields.contains_key("pid")
    }

    fn is_valid(&self) -> bool {
        self.validate_byr()
            && self.validate_iyr()
            && self.validate_eyr()


    }

    fn validate_byr(&self) -> bool {
        self.byr.parse::<u32>().map_or(false, |n| 1920 <= n && n <= 2002)
    }

    fn validate_iyr(&self) -> bool {
        self.iyr.parse::<u32>().map_or(false, |n| 2010 <= n && n <= 2020)
    }

    fn validate_eyr(&self) -> bool {
        self.eyr.parse::<u32>().map_or(false, |n| 2020 <= n && n <= 2030)
    }
}

pub struct Day4Solve {}

impl Solution for Day4Solve {

    fn part_1(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        Ok(load_passports(input).len().to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let count = load_passports(input).iter()
            .filter(|p| p.is_valid())
            .count();
        Ok(count.to_string())
    }
}


fn load_passports(input: &mut dyn BufRead) -> Vec<Passport> {
    let mut passports: Vec<Passport> = Vec::new();
    let mut lines_iter = input.lines().into_iter();

    while let Some(line) = lines_iter.next() {
        let mut line = line.unwrap();

        let mut fields: HashMap<String, String> = HashMap::new();
        loop {
            let line_fields = line.split_whitespace()
                .map(|s| s.split(":"))
                .map(|mut split| (split.next().unwrap(), split.next().unwrap()));

            for (key, value) in line_fields {
                fields.insert(key.to_owned(), value.to_owned());
            }

            // Break loop is next line is blank or no more lines left
            line = match lines_iter.next() {
                Some(Ok(l)) if !l.is_empty() => l,
                _ => break,
            }
        }

        Passport::from_fields(fields).map(|p| passports.push(p));
    }

    passports
}


