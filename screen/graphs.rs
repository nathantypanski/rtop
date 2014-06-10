use ncurses;
use std::vec::Vec;
use screen::display;

#[deriving (Clone)]
pub struct Bar {
    ratio: i32,
}

pub struct Graph {
    bars: Vec<Bar>,
    linecount: i32,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            bars: Vec::new(),
            linecount: 0,
        }
    }

    pub fn draw_bar(bar: &Bar, linecount: i32) {
        let height = 10;
        let yoffset = 2;
        let yloc = height - bar.ratio;
        ncurses::mvvline(yoffset + yloc, linecount, ('|' as u32), bar.ratio);
    }

    pub fn add_bar(&mut self, percent: uint) {
        self.bars.push(Bar { ratio: (percent / 10) as i32 });
    }

    pub fn render(&mut self) {
        let (max_x, max_y) = display::get_dimensions();
        while (self.bars.len() > (max_x - 2) as uint) {
            let _ = self.bars.shift();
        }
        self.linecount = 0;
        draw_rect_fill(0, 1, max_x, 11);
        draw_rect(0, 1, max_x, 12);
        for ref mut bar in self.bars.iter() {
            Graph::draw_bar(&bar.clone(), (self.linecount) as i32);
            self.linecount += 1;
        }
        self.linecount = 0;
        ncurses::refresh();
    }
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
