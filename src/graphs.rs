use std::vec::Vec;
use std::comm;

use ncurses;

use display;

#[deriving (Clone)]
pub struct Bar {
    ratio: i32,
}

pub fn hook(rx: Receiver<int>) -> Sender<uint> {
    let (shutdown_tx, shutdown_rx) = comm::channel();
    spawn(proc() {
        let mut bars = box Vec::new();
        loop {
            let value = rx.recv();
            bars.push(Bar { ratio: (value / 10) as i32 });
            if shutdown_rx.try_recv().is_ok() { break }
            render(bars.clone());
        }
    });
    shutdown_tx
}

pub fn render(mut bars: Box<Vec<Bar>>) {
    let (max_x, _) = display::get_dimensions();
    while bars.len() > (max_x - 2) as uint {
        let _ = bars.shift();
    }
    let mut linecount = 0u;
    draw_rect_fill(0, 1, max_x, 11);
    draw_rect(0, 1, max_x, 12);
    for ref mut bar in bars.iter() {
        draw_bar(&bar.clone(), (linecount) as i32);
        linecount += 1;
    }
    linecount = 0;
    ncurses::refresh();
}

fn draw_bar(bar: &Bar, linecount: i32) {
    let height = 10;
    let yoffset = 2;
    let yloc = height - bar.ratio;
    ncurses::mvvline(yoffset + yloc, linecount, ('|' as u32), bar.ratio);
}

pub fn draw_rect(x1: i32, y1: i32, x2: i32, y2: i32) {
    ncurses::mvhline(y1, x1, ('-' as u32), x2 - x1);
    ncurses::mvhline(y2, x1, ('-' as u32), x2 - x1);
    ncurses::mvvline(y1, x1, ('|' as u32), y2 - y1 + 1);
    ncurses::mvvline(y1, x2 - 1, ('|' as u32), y2 - y1 + 1);
    ncurses::mvaddch(y1, x1, '+' as u32);
    ncurses::mvaddch(y2, x1, '+' as u32);
    ncurses::mvaddch(y2, x2 - 1, '+' as u32);
    ncurses::mvaddch(y1, x2 - 1, '+' as u32);
    ncurses::refresh();
}

pub fn draw_rect_fill(x1: i32, y1: i32, x2: i32, y2: i32) {
    for y in range(y1, y2) {
        ncurses::mvhline(y, x1, (' ' as u32), x2 - x1);
    }
}
