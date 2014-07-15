use std::num;
use std::io::{File, Open, Read, SeekSet};
use std::comm::Receiver;
use std::ascii::AsciiCast;
use std::comm;
use std::io::timer;

pub struct CpuReader<'a> {
    path: &'a Path,
}

impl<'a> CpuReader<'a> {
    pub fn new<'b>(path: &'b Path) -> CpuReader<'b> {
        CpuReader {
            path: path,
        }
    }

    pub fn listen(&mut self) -> Receiver<int> {
        let path = self.path.clone();
        let (tx, rx) = comm::channel();
        spawn(proc() {
            let mut file = match File::open_mode(&path, Open, Read) {
                Ok(file) => { file }
                Err(_) => { fail!("Couldn't open CPU in procfs!") }
            };
            let file = &mut file;
            loop {
                let cpu1 = read_stat(file).expect("");
                timer::sleep(1000);
                let cpu2 = read_stat(file).expect("");
                let usage = cpu2.usage(cpu1);
                if tx.send_opt(usage).is_err() { break }
            }
        });
        rx
    }

}

pub fn read_stat(file: &mut File) -> Option<Cpu> {
    let bytes = file.read_to_end().unwrap();
    let move_bytes = bytes.move_iter();
    let contents_bytes: Vec<Ascii> = move_bytes
        .take_while(|c| c != &('\n' as u8))
        .map(|c| c.to_ascii())
        .collect();
    let contents = contents_bytes.into_string();
    let re = regex!(r"cpu\s+([0-9]+)\s+([0-9]+)\s+([0-9]+)\s+([0-9]+)\s+([0-9]+)\s+([0-9]+)\s+([0-9]+).*");
    let cpu = match re.captures(contents.as_slice()) {
        Some(matches) => {
            Some(Cpu::new(
                num::from_str_radix(matches.at(1), 10).expect(""), // user
                num::from_str_radix(matches.at(2), 10).expect(""), // nice
                num::from_str_radix(matches.at(3), 10).expect(""), // system
                num::from_str_radix(matches.at(4), 10).expect(""), // idle
                num::from_str_radix(matches.at(5), 10).expect(""), // iowait
                num::from_str_radix(matches.at(6), 10).expect(""), // irq
                num::from_str_radix(matches.at(7), 10).expect("")) // softirq
                 )
        },
        None => {
            None
        }
    };
    let _ = file.seek(0, SeekSet);
    cpu
}

#[deriving(Show)]
pub struct Cpu {
    pub user: int,
    pub nice: int,
    pub system: int,
    pub idle: int,
    pub iowait: int,
    pub irq: int,
    pub softirq: int,
    pub total: int,
    pub work: int,
}

impl Cpu {
    pub fn new(user: int, nice: int, system: int, idle: int,
               iowait: int, irq: int, softirq: int) -> Cpu {
        let total = user + nice + system + idle + iowait + irq + softirq;
        let work = user + nice + system;
        Cpu {
            user: user,
            nice: nice,
            system: system,
            idle: idle,
            iowait: iowait,
            irq: irq,
            softirq: softirq,
            total: total,
            work: work,
        }
    }

    pub fn usage(&self, prev: Cpu) -> int {
        let diffidle = self.idle - prev.idle;
        let difftotal = self.total - prev.total;
        let mut diffusage = -1;
        if difftotal > 0 {
            diffusage = (1000 * (difftotal - diffidle) / difftotal + 5) / 10;
        }
        diffusage
    }
}
