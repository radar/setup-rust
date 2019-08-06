extern crate ansi_term;

use ansi_term::Colour;

pub fn found(text: &str) {
  println!("{}", Colour::Green.paint(text))
}

pub fn fail(text: &str) {
  println!("{}", Colour::Red.paint(text))
}

pub fn info(text: &str) {
  println!("{}", Colour::Yellow.paint(text))
}
