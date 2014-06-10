/*
 *
 * ██████╗ ████████╗ ██████╗ ██████╗
 * ██╔══██╗╚══██╔══╝██╔═══██╗██╔══██╗
 * ██████╔╝   ██║   ██║   ██║██████╔╝
 * ██╔══██╗   ██║   ██║   ██║██╔═══╝
 * ██║  ██║   ██║   ╚██████╔╝██║
 *
 * rtop
 *
 * A system monitor written in Rust.
 *
 * Copyright © 2014 Nathan Typanski <{mail} at nathantypanski.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */


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
