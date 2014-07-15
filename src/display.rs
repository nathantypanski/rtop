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
    ncurses::start_color();
    ncurses::init_pair(1, ncurses::COLOR_WHITE, ncurses::COLOR_WHITE);
    ncurses::init_pair(2, ncurses::COLOR_BLUE, ncurses::COLOR_BLUE);
    ncurses::init_pair(3, ncurses::COLOR_GREEN, ncurses::COLOR_GREEN);
    ncurses::init_pair(5, ncurses::COLOR_RED, ncurses::COLOR_RED);
    ncurses::init_pair(7, ncurses::COLOR_YELLOW, ncurses::COLOR_YELLOW);
    ncurses::refresh();
    get_dimensions()
}

/*
 * Get the display dimensions (x, y)
 */
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
    ncurses::refresh();
    ncurses::endwin();
}
