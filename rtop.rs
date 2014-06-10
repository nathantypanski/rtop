#![feature(globs)]

extern crate regex;
extern crate collections;
extern crate ncurses;

use ncurses::*;

use collections::string::String;
use regex::Regex;

use std::io::fs;
use std::path::posix::Path;

fn is_proc(dir: &Path) -> Option<Path> {
    let mut result = None;
    if dir.is_dir() {
        let fname_str = dir.filename_str().expect("");
        let re = match Regex::new(r"^[0-9]+$") {
            Ok(re) => re,
            Err(err) => fail!("{}", err),
        };
        result = match re.is_match(fname_str) {
            true => Some(dir.clone()),
            false => None,
        };
    }
    result
}

fn read_meminfo(meminfo: &Path) -> Option<String> {
    let mut file = fs::File::open(meminfo);
    match file.read_to_end() {
        Ok(bytes) => {
            bytes.into_ascii_opt().map(|b| b.into_str())
        },
        Err(_) => { None },
    }
}

fn main() {
    let procfs: Path = from_str("/proc").expect("Must have access to procfs!");
    let meminfo: Path = from_str("/proc/meminfo").expect("Must have access to procfs!");
    match fs::readdir(&procfs) {
        Ok(diri) => {
            for dir in diri.iter() {
                match is_proc(dir) {
                    Some(procdir) => {
                        println!("{}", procdir.filename_display());
                    }
                    None => {}
                }
            }

        }
        Err(_) => {}
    }
    println!("{}", read_meminfo(&meminfo));

  /* Start ncurses. */
  initscr();

  /* Print to the back buffer. */
  printw("Hello, world!");

  /* Update the screen. */
  refresh();

  /* Wait for a key press. */
  getch();

  /* Terminate ncurses. */
  endwin();
}
