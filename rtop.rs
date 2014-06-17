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


use std::io::timer::sleep;
use std::comm::channel;
use std::path::posix::Path;

use cpu::CpuReader;
use memory::read_meminfo;
use screen::{Graph, screen_init, screen_die, draw_rect};

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

    let mut graph: Graph = screen::Graph::new();
    let mut cpureader = CpuReader::new(&procstat);
    loop {
        let cpu1 = cpureader.read_stat().expect("");
        sleep(100);
        let cpu2 = cpureader.read_stat().expect("");
        sleep(300);
        let usage = cpu2.usage(cpu1);
        graph.add_bar(usage as uint);
        graph.render();
        ncurses::mvprintw(0, 0, format!("{}", usage).as_slice());
        // ncurses::mvvline(1, 0, (' ' as u32), 10);
        // ncurses::mvvline(1, 0, ('|' as u32), (usage % 10) as i32);

        match receiver.try_recv() {
            Ok(_) => { break },
            _ => {},
        }
        ncurses::refresh();
    }

    screen_die();
}
