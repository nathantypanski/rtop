extern crate log;
use std::num;
use std::owned::Box;
use std::io::{BufferedReader, File, Open, Read, SeekSet};
use std::ascii::AsciiCast;
use log;

use self::types::Cpu;

mod types;

pub struct CpuReader<'a> {
    path: &'a Path,
    file: Box<File>,
}

impl<'a> CpuReader<'a> {
    pub fn new<'b>(path: &'b Path) -> CpuReader<'b> {
        let file = File::open_mode(path, Open, Read).unwrap();
        CpuReader {
            path: path,
            file: box file,
        }
    }

    pub fn read_stat(&mut self) -> Option<Cpu> {
        let bytes = self.file.read_to_end().unwrap();
        let move_bytes = bytes.move_iter();
        let contents_bytes: Vec<Ascii> = move_bytes
            .take_while(|c| c != &('\n' as u8))
            .map(|c| c.to_ascii())
            .collect();
        let contents = contents_bytes.into_string();
        debug!("{}", contents);
        let re = regex!(r"cpu\s+([0-9]+)\s+([0-9]+)\s+([0-9]+)\s+([0-9]+)\s+([0-9]+)\s+([0-9]+)\s+([0-9]+).*");
        let cpu = match re.captures(contents.as_slice()) {
            Some(matches) => {
                Some(Cpu::new(
                    num::from_str_radix(matches.at(1), 10).expect(""),
                    num::from_str_radix(matches.at(2), 10).expect(""),
                    num::from_str_radix(matches.at(3), 10).expect(""),
                    num::from_str_radix(matches.at(4), 10).expect(""),
                    num::from_str_radix(matches.at(5), 10).expect(""),
                    num::from_str_radix(matches.at(6), 10).expect(""),
                    num::from_str_radix(matches.at(7), 10).expect(""))
                     )
            },
            None => {
                None
            }
        };
        let _ = self.file.seek(0, SeekSet);
        cpu
    }
}
