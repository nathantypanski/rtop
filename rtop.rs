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
#[phase(syntax)]

extern crate regex_macros;
extern crate regex;
extern crate collections;
extern crate ncurses;


use std::io::timer::sleep;
use std::comm::channel;
use std::path::posix::Path;

use cpu::read_stat;
use memory::read_meminfo;
use screen::{screen_init, screen_die};

mod processes;
mod memory;
mod cpu;
mod screen;

fn main() {
    let procfs: Path = from_str("/proc").expect("Must have access to procfs!");
    let meminfo: Path = from_str("/proc/meminfo").expect("Must have access to procfs!");
    let procstat: Path = from_str("/proc/stat").expect("Must have access to procfs!");

    println!("{}", read_meminfo(&meminfo));

    let (max_x, max_y) = screen_init();

    let (sender, receiver) = channel();

    spawn(proc() {
        loop {
            let ch = ncurses::getch();
            sender.send(ch);
            break;
        }
    });

    loop {
        let cpu1 = read_stat(&procstat).expect("");
        sleep(100);
        let cpu2 = read_stat(&procstat).expect("");
        sleep(300);
        let usage: int = cpu2.usage(cpu1);
        ncurses::mvprintw(0, 0, format!("{}", usage).as_slice());
        ncurses::mvvline(1, 0, (' ' as u32), 10);
        ncurses::mvvline(1, 0, ('|' as u32), (usage % 10) as i32);

        match receiver.try_recv() {
            Ok(_) => { break },
            _ => {},
        }
        ncurses::refresh();
    }

    screen_die();
}
