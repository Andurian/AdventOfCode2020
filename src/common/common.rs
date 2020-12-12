use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;

pub fn parse_file_linewise<TRet, F>(filename: &str, f: F) -> Vec<TRet>
where
    F: Fn(&str) -> TRet,
{
    fs::read_to_string(filename)
        .unwrap()
        .split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| f(line))
        .collect::<Vec<TRet>>()
}

pub fn parse_file_linewise_as<T: FromStr>(filename: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    parse_file_linewise(filename, |line| line.parse::<T>().unwrap())
}

pub fn read_file_linewise(filename: &str) -> Vec<String> {
    parse_file_linewise(filename, |s| String::from(s))
}

pub fn read_grouped_file(filename: &str) -> Vec<String> {
    let reader = BufReader::new(File::open(filename).unwrap());

    let mut ret = Vec::<String>::new();
    let mut current = String::new();
    for line in reader.lines() {
        let line = String::from(line.unwrap().trim());
        if line.is_empty() {
            if !current.is_empty() {
                ret.push(current.trim().to_string());
            }
            current = String::new();
        } else {
            current.push('\n');
            current.push_str(&line);
        }
    }

    if !current.is_empty() {
        ret.push(current.trim().to_string());
    }

    ret
}
