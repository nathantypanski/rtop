use ncurses;

/*
 * Display initializaiton.
 */
pub fn screen_init() -> (i32, i32) {
    ncurses::initscr();
    ncurses::raw();
    ncurses::keypad(ncurses::stdscr, true);
    ncurses::noecho();
    ncurses::curs_set(ncurses::CURSOR_INVISIBLE);
    ncurses::refresh();
    get_dimensions()
}

pub fn get_dimensions() -> (i32, i32) {
    let mut max_x = 0;
    let mut max_y = 0;
    ncurses::getmaxyx(ncurses::stdscr, &mut max_y, &mut max_x);
    (max_x, max_y)
}

/*
 * Display teardown.
 */
pub fn screen_die() {
    ncurses::endwin();
}
