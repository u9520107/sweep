extern crate regex;
extern crate getopts;


use std::io::fs;
use std::io::fs::PathExtensions;
use std::os;

use regex::Regex;
use std::io;

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

fn print_usage(program: &str) {
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
            //based on mdn's escape regular expression function
            "["|"."|"+"|"?"|"^"|"$"|"{"|"}"|"("|")"|"]"|"|"|"\\" => {
                result.push_str("\\");
                result.push_str(g);
            },
            //treat * separately
            "*" => {
                result.push_str(".*?");
            },
            _ => result.push_str(g),
        }
    }
    //add '$' to regex if the pattern doesn't end with *
    if !pattern.ends_with("*") {
        result.push('$');
    }

    Regex::new(result.as_slice()).unwrap()
}

fn sweep(path: &str, pattern: &str) {
    let p = Path::new(path);
    println!("sweeping in {} for {}", p.display(), pattern);

    //convert pattern to regular expression
    let re = pattern_to_regex(pattern);

    let mut folders = fs::walk_dir(&p).unwrap();

    for f in folders {
       if f.is_file() && re.is_match(f.filename_str().unwrap()) {
           println!("removing {}", f.display());
            fs::unlink(&f).unwrap();
       }
    }
}


#[test]
fn test_sweep() {
    //create files and folders
    let p = Path::new("./tmp/nested/folder");
    fs::mkdir_recursive(&p, io::USER_RWX).unwrap();

    {
        fs::File::open_mode(&Path::new("./tmp/test.txt"), io::Open, io::Write).unwrap().write_line("test").unwrap();
        fs::File::open_mode(&Path::new("./tmp/test.swp"), io::Open, io::Write).unwrap().write_line("test").unwrap();
        fs::File::open_mode(&Path::new("./tmp/nested/test.txt"), io::Open, io::Write).unwrap().write_line("test").unwrap();
        fs::File::open_mode(&Path::new("./tmp/nested/test.swp"), io::Open, io::Write).unwrap().write_line("test").unwrap();
        fs::File::open_mode(&Path::new("./tmp/nested/folder/test.txt"), io::Open, io::Write).unwrap().write_line("test").unwrap();
        fs::File::open_mode(&Path::new("./tmp/nested/folder/test.swp"), io::Open, io::Write).unwrap().write_line("test").unwrap();
    }

    sweep("./tmp", "*.swp");
    assert!(!Path::new("./tmp/test.swp").exists());
    assert!(!Path::new("./tmp/nested/test.swp").exists());
    assert!(!Path::new("./tmp/nested/folder/test.swp").exists());

    assert!(Path::new("./tmp/test.txt").exists());
    assert!(Path::new("./tmp/nested/test.txt").exists());
    assert!(Path::new("./tmp/nested/folder/test.txt").exists());

    fs::rmdir_recursive(&p).unwrap();
}
