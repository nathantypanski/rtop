
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
