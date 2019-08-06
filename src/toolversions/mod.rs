use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

use std::path::Path;

pub fn present() -> bool {
  path().exists()
}

pub fn parse<'a>() -> Vec<Vec<&'a str>> {
    let f = BufReader::new(file().unwrap());
    return f.lines().map(|line| line.unwrap().split_whitespace().collect()).collect();

}

fn file() -> io::Result<File> {
    let f = File::open(path())?;
    return Ok(f)
}


fn path<'a>() -> &'a Path {
  return Path::new(".tool-versions");
}
