use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Debug)]
pub enum ParsePasswordEntryError {
    Policy,
    Password,
}

impl Display for ParsePasswordEntryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            ParsePasswordEntryError::Policy => write!(f, "Policy error"),
            ParsePasswordEntryError::Password => write!(f, "Password error"),
        }
    }
}

#[derive(Debug)]
struct PasswordPolicy {
    min: u32,
    max: u32,
    character: char,
}

#[derive(Debug)]
pub struct PasswordEntry {
    policy: PasswordPolicy,
    password: String,
}

impl FromStr for PasswordEntry {
    type Err = ParsePasswordEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PasswordEntry {
            policy: PasswordPolicy {
                min: 1,
                max: 2,
                character: 'a',
            },
            password: String::from("Test"),
        })
    }
}

pub fn parse_password(line: &str) -> Result<PasswordEntry, ParsePasswordEntryError> {
    line.parse()
}

pub fn day_two() {
    let file = File::open("input/2.txt").expect("Could not read day 2 input");
    let reader = BufReader::new(file);

    let entries: Vec<PasswordEntry> = reader
        .lines()
        .map(|line_result| {
            let line = line_result.expect("Failed to read line");
            parse_password(line.as_str()).expect("Failed to parse entry")
        })
        .collect();

    for entry in entries.iter() {
        println!("{:?}", entry);
    }
}
