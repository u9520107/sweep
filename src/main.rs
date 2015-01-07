extern crate regex;
extern crate getopts;

use std::io::fs;
use std::io::fs::PathExtensions;
use std::os;

fn main () {
    let args: Vec<String> = os::args();

    let program = args[0].as_slice();

    if args.len() != 3 {
        print_usage(program);
    } else {
        let p = args[1].as_slice();
        let pat = args[2].as_slice();
        sweep(p, pat);
    }
}

fn print_usage (program: $str) {
    println!("Usage {} folderpath pattern", program);
}

fn pattern_to_regex(pattern: &str) -> Regex {
    let mut result = String::new();
    //add '^' to regex if the pattern doesn't start with *
    if !pattern.starts_with("*") {
        result.push('^');
    }

    for g in pattern.graphemes(true) {
        match g {
            //based on mdn's escapse regular expression function

        }
    }
}
