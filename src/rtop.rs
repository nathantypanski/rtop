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
#![feature(phase)]
#![allow(ctypes)]

extern crate regex;
extern crate collections;
extern crate ncurses;
#[phase(plugin, link)] extern crate regex_macros;
#[phase(plugin, link)] extern crate log;


use std::comm::channel;
use std::path::posix::Path;

use procfs::cpu::CpuReader;

mod procfs;
mod graphs;
mod display;

fn main() {
    let procfs = from_str::<Path>("/proc").unwrap();
    let meminfo = procfs.join(from_str::<Path>("meminfo").unwrap());
    let procstat = procfs.join(from_str::<Path>("/proc/stat").unwrap());

    let (_, _) = display::screen_init();

    let (keypress_tx, keypress_rx) = channel();
    spawn(proc() {
        let ch = ncurses::getch();
        keypress_tx.send(ch);
    });

    let cpu_sd = {
        let mut cpureader = CpuReader::new(&procstat);
        graphs::hook(cpureader.listen(), Some("CPU".to_string()))
    };
    keypress_rx.recv();
    cpu_sd.send(1u);
    display::screen_die();
    //println!("{}", processes::read_processes(&procfs));
}
