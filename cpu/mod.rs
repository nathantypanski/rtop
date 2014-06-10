use std::num;
use std::io::{BufferedReader, File};

use self::types::Cpu;

mod types;

/*
 * Read /proc/stat
 */
pub fn read_stat(stat: &Path) -> Option<Cpu> {
    let file = File::open(stat);
    let mut reader = BufferedReader::new(file);
    let re = regex!(r"cpu\s+([0-9]+)\s+([0-9]+)\s+([0-9]+)\s+([0-9]+)\s+([0-9]+)\s+([0-9]+)\s+([0-9]+).*");
    let cpu = match reader.read_line() {
        Ok(bytes) => {
            let line = bytes.to_str();
            match re.captures(line.as_slice()) {
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
            }
        },
        Err(_) => {
            None
        },
    };
    cpu
}
