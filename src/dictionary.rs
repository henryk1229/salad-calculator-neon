// use std::collections::HashSet;
// use std::process::exit;
// use trie::Trie;

use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
};

pub fn build_dictionary() -> Filter<Map<std::str::Split<'_, &str>>> {
    let word_list = include_str!("./word_list/words_alpha.txt")
    .split("\r\n")
    .map(|str| str.to_string())
    // filter out all words less than 3 characters, and any word that has a q not followed by a u (or that ends in q)
    .filter(|ref line| {
      if line.len() < 3 {
        return false;
      }

      let mut iter = line.chars();
      while let Some(c) = iter.next() {
        if c == 'q' {
          if let Some(n) = iter.next() {
            if n != 'u' {
              // q not followed by u
              return false;
            }
          } else {
            // line ends in q
            return false;
          }
        }
      }

      true
    });

    word_list
}

fn main() -> Result<(), Err> {
    let dict = build_dictionary();
    println!("dict: {:?}", dict);
    Ok(())
}