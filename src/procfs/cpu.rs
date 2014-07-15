use std::num;
use std::io::{File, Open, Read, SeekSet};
use std::comm::Receiver;
use std::ascii::AsciiCast;
use std::comm;
use std::io::timer;

static CPU_POLL_SLEEP: u64 = 100u64;

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
            let mut file = File::open_mode(&path, Open, Read).unwrap();
            let file = &mut file;
            loop {
                let cpu1 = read_stat(file);
                timer::sleep(CPU_POLL_SLEEP);
                let cpu2 = read_stat(file);
                let usage = cpu2.usage(cpu1);
                if tx.send_opt(usage).is_err() { break }
            }
        });
        rx
    }

}

pub fn read_stat(file: &mut File) -> Cpu {
    let bytes: Vec<u8> = file.read_to_end().unwrap();
    let contents: Vec<String> = bytes
        .move_iter()
        .take_while(|c| c != &('\n' as u8))
        .map(|c| c.to_ascii())
        .collect::<Vec<_>>()
        .into_string()
        .as_slice()
        .split(' ')
        .filter_map(|a|
            if a.len() > 0 {
                Some(a.trim_chars(' ').into_string()) }
            else {
                None
            })
        .collect();
    let cpu = Cpu::new(
                num::from_str_radix(contents.get(1).as_slice(), 10).unwrap(),  // user
                num::from_str_radix(contents.get(2).as_slice(), 10).unwrap(),  // nice
                num::from_str_radix(contents.get(3).as_slice(), 10).unwrap(),  // system
                num::from_str_radix(contents.get(4).as_slice(), 10).unwrap(),  // idle
                num::from_str_radix(contents.get(5).as_slice(), 10).unwrap(),  // iowait
                num::from_str_radix(contents.get(6).as_slice(), 10).unwrap(),  // irq
                num::from_str_radix(contents.get(7).as_slice(), 10).unwrap()); // softirq
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
    pub fn new(user: int,
               nice: int,
               system: int,
               idle: int,
               iowait: int,
               irq: int,
               softirq: int)
               -> Cpu
    {
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
