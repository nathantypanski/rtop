use std::vec::Vec;
use std::comm;

use ncurses;

use display;

pub fn hook(rx: Receiver<int>, title: Option<String>) -> Sender<uint> {
    let (shutdown_tx, shutdown_rx) = comm::channel();
    spawn(proc() {
        let mut bars = box Vec::new();
        loop {
            let value = rx.recv();
            bars.push((value / 10) as i32 );
            if shutdown_rx.try_recv().is_ok() { break }
            render(bars.clone(), title.clone().map(|x| x + ": " + value.to_string()));
        }
    });
    shutdown_tx
}

fn render(mut bars: Box<Vec<i32>>, title: Option<String>) {
    let (max_x, _) = display::get_dimensions();
    while bars.len() > (max_x - 3) as uint {
        let _ = bars.remove(0);
    }
    let mut linecount = 0u;
    draw_rect_fill(1, 0, max_x, 12);
    draw_rect(1, 0, max_x, 12, title);
    for ref mut bar in bars.iter() {
        draw_bar(bar.clone(), linecount as i32);
        linecount += 1;
    }
    ncurses::refresh();
}

fn draw_bar(bar: i32, linecount: i32) {
    let height = 10;
    let yoffset = 2;
    let yloc = height - bar;
    let graph_char = '|' as u32;
    let color = match bar {
        x if x > 7 => 5,
        x if x > 4 => 7,
        _ => 3,
    };
    ncurses::attron(ncurses::COLOR_PAIR(color));
    ncurses::mvvline(yoffset + yloc, linecount + 2, graph_char, bar);
    ncurses::attroff(ncurses::COLOR_PAIR(color));
}

fn draw_rect(x1: i32, y1: i32, x2: i32, y2: i32, title: Option<String>) {
    ncurses::mvhline(y1, x1, ('-' as u32), x2 - x1);
    ncurses::mvhline(y2, x1, ('-' as u32), x2 - x1);
    match title {
        Some(title) => {
            let title_lspace = 3i32;
            ncurses::mvprintw(y2,
                              x1 + title_lspace,
                              ("[".to_string() + title + "]".to_string())
                              .as_slice()
                              );
        }
        _ => {}
    }
    ncurses::mvvline(y1, x1, ('|' as u32), y2 - y1 + 1);
    ncurses::mvvline(y1, x2 - 1, ('|' as u32), y2 - y1 + 1);
    ncurses::mvaddch(y1, x1, '+' as u32);
    ncurses::mvaddch(y2, x1, '+' as u32);
    ncurses::mvaddch(y2, x2 - 1, '+' as u32);
    ncurses::mvaddch(y1, x2 - 1, '+' as u32);
}

fn draw_rect_fill(x1: i32, y1: i32, x2: i32, y2: i32) {
    for y in range(y1, y2) {
        ncurses::mvhline(y, x1, (' ' as u32), x2 - x1);
    }
}
