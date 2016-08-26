extern crate chrono;

use std::fs::File;

use std::io::prelude::*;

use std::path::Path;

use chrono::naive::date::NaiveDate;

#[derive(Debug)]
struct Entry {
    date: NaiveDate,
    ver: Vec<i8>,
}

// Result<> giving version of the function below
// fn read_changelog_m(fp: &str) -> Result<String, String> {
//     let path = &Path::new(fp);
//     let mut s = String::new();

//     File::open(path)
//         .and_then( |mut f| f.read_to_string(&mut s) )
//         .map_err( |e| e.to_string() )
//         .map( |_| s )
// }

fn read_changelog(fp: &str) -> String {
    let path = &Path::new(fp);
    let mut s = String::new();

    match File::open(path).and_then(|mut f| f.read_to_string(&mut s)) {
        Ok(_) => s,
        Err(err) => panic!(err),
    }
}

// Result<> giving version of the function below
// fn parse_date_m(s: &str) -> Result<NaiveDate, String> {
//     let d: Vec<&str> = s.split_at(2).1.split_whitespace().take(4).collect();

//     NaiveDate::parse_from_str(&d.concat(), "%a%h%d%Y")
//         .map_err( |e| format!("Date Parse Error: {} in {}", e, s) )
// }

fn parse_date(s: &str) -> NaiveDate {
    let d: Vec<&str> = s.split_at(2).1.split_whitespace().take(4).collect();
    match NaiveDate::parse_from_str(&d.concat(), "%a%h%d%Y") {
        Ok(d) => d,
        Err(e) => {
            println!("Date Parse Error: {} in {}", e, s);
            std::process::exit(1)
        }
    }
}

// This is a Result<> returning version of the function below as a
// thought experiment to see what it's like to carry error info
// around and roll it up into a final result.
//
// fn parse_ver_m(s: &str) -> Result<Vec<i8>, String> {
//     s.split_whitespace()
//         .last()
//         .ok_or("Couldn't find Version string".to_owned())
//         .and_then(|v_str| {
//             v_str.split(".")
//                 .map(|c| {
//                     c.parse::<i8>()
//                         .map_err(|e| e.to_string())
//                 })
//                 .collect()
//         })
// }

// Terrifying version...
fn parse_ver(s: &str) -> Vec<i8> {
    let v_str = s.split_whitespace().last().unwrap();
    v_str.split(".").map(|c| c.parse::<i8>().unwrap()).collect()
}

fn chk_entry(a: &Entry, b: &Entry) {
    if a.date < b.date {
        println!("Changelog dates not in order");
        std::process::exit(1)
    } else if a.ver < b.ver {
        println!("Changelog versions not in order");
        std::process::exit(1)
    }
}

fn main() {
    let s = read_changelog(&std::env::args().nth(1).unwrap());

    let l: Vec<Entry> = s.lines()
        .filter(|x| x.starts_with("*"))
        .map(|x| {
            Entry {
                date: parse_date(x),
                ver: parse_ver(x),
            }
        })
        .collect();

    let mut iter = l.iter().peekable();

    while !iter.peek().is_none() {
        match (iter.next(), iter.peek()) {
            (Some(e), Some(ne)) => chk_entry(e, ne),
            _ => std::process::exit(0),
        }
    }
}
