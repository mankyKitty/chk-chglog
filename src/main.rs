extern crate chrono;

use std::fs::File;
use std::cmp::*;
use std::io::prelude::*;

use std::path::Path;

use chrono::naive::date::NaiveDate;

struct Entry {
    line_num: usize,
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
//     NaiveDate::parse_from_str(&s[2..17], "%a %h %d %Y")
//         .map_err( |e| format!("Date Parse Error: {} in {}", e, s) )
// }

fn parse_date(ln: usize, s: &str) -> NaiveDate {
    match NaiveDate::parse_from_str(&s[2..17], "%a %h %d %Y") {
        Ok(d) => d,
        Err(e) => {
            println!("Date Parse Error: {} in {}, line:{}", e, s, ln);
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
fn parse_ver(_ln: usize, s: &str) -> Vec<i8> {
    let v_str = s.split_whitespace().last().unwrap();
    v_str.split(".").map(|c| c.parse::<i8>().unwrap()).collect()
}

#[test]
fn compare_ver_expect_false() {
    let a = vec![1, 2, 0];
    let b = vec![1, 1, 0];

    assert_eq!(compare_ver(&a,&b), Ordering::Greater);
}
#[test]
fn compare_ver_edge() {
    let a = vec![1, 3, 0];
    let b = vec![1, 2, 5];
    assert_eq!(compare_ver(&a,&b), Ordering::Greater);
}
#[test]
fn compare_ver_catch() {
    let a = vec![1, 3, 0];
    let b = vec![1, 3, 5];
    assert_eq!(compare_ver(&a,&b), Ordering::Less);
}
#[test]
fn compare_ver_ummm() {
    let a = vec![1, 3, 0, 4];
    let b = vec![1, 3, 5];
    assert_eq!(compare_ver(&a,&b), Ordering::Less);
}
#[test]
fn compare_ver_wut() {
    let a = vec![1, 3, 4];
    let b = vec![1, 3, 0, 5];
    assert_eq!(compare_ver(&a,&b), Ordering::Greater);
}

fn ver_pair_compare(prev: Ordering, (a,b): (&i8, &i8) ) -> Ordering {
    match prev {
        Ordering::Less => Ordering::Less,
        Ordering::Equal => Ord::cmp(&a, &b),
        Ordering::Greater => Ordering::Greater
    }
}

fn compare_ver(a: &Vec<i8>, b: &Vec<i8> ) -> Ordering {
    a.iter().zip(b.iter()).fold(Ordering::Equal, ver_pair_compare )
}

fn show_ver(v: &Vec<i8>) -> String {
    let s: Vec<String> = v.iter().map( |n| { n.to_string() } ).collect();
    s.join(".")
}

fn ooo_err_die(typ: &str, cname: &str, a_v: String, b_v: String, a_n: usize, b_n: usize) {
    println!(
        "[{}]: {} out of order: {} < {}, Lines: {},{}",
        cname, typ, a_v, b_v, a_n, b_n
    );
    std::process::exit(1)
}

fn chk_entry(cname: &str, a: &Entry, b: &Entry) {
    if a.date < b.date {
        ooo_err_die(
            "Dates",
            cname,
            a.date.to_string(),
            b.date.to_string(),
            a.line_num,
            b.line_num
        )

    } else if compare_ver(&a.ver, &b.ver) == Ordering::Less {
        ooo_err_die(
            "Versions",
            cname,
            show_ver( &a.ver ),
            show_ver( &b.ver ),
            a.line_num,
            b.line_num
        )
    }
}

fn main() {
    let cname = &std::env::args().nth(1).unwrap();
    let s = read_changelog(cname);

    let l: Vec<Entry> = s.lines().enumerate()
        .filter(|x| x.1.starts_with("*"))
        .map(|x| {
            let ln = x.0 + 1;
            Entry {
                line_num: ln,
                date: parse_date(ln, x.1),
                ver: parse_ver(ln, x.1),
            }
        })
        .collect();

    let mut iter = l.iter().peekable();

    while !iter.peek().is_none() {
        match (iter.next(), iter.peek()) {
            (Some(e), Some(ne)) => chk_entry(cname, e, ne),
            _ => std::process::exit(0),
        }
    }
}
