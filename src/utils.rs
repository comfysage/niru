use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_absolute_path(path: PathBuf) -> Result<String, String> {
    match path.to_str() {
        None => Err("couldn't unwrap path.".into()),
        Some(str) => Ok(str.to_string()),
    }
}

fn get_home() -> String {
    let path: &'static str = std::env!("HOME");
    path.to_string()
}

pub fn extend_path(path: &String) -> String {
    let mut str = path.clone();
    if str.as_bytes()[0] as char == '~' {
        str = get_home() + &str[1..]
    }
    str
}
