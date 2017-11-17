
// Prevents a spare console from being created attached to our program on
// windows, but only if we're running in release mode.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate easycurses;

use easycurses::*;

fn main() {
    // Common startup
    let mut easy = EasyCurses::initialize_system().unwrap();
    easy.set_cursor_visibility(CursorVisibility::Invisible);
    easy.set_echo(false);
    easy.set_keypad_enabled(true);

    // Check the size of the window.
    let (row_count, col_count) = easy.get_row_col_count();

    // A message using RC coordinates.
    easy.move_rc(0, 0);
    assert_eq!(easy.get_cursor_rc(), (0, 0));
    easy.print("Hello from RC 0,0.");

    // A message using XY coordinates.
    easy.move_xy(1, 1);
    assert_eq!(easy.get_cursor_xy(), (1, 1));
    easy.print("Hello from XY 1,1.");

    // Upper right corner has a '+'
    easy.move_rc(0, col_count - 1);
    easy.print_char('+');

    // Lower left corner has a '-', note that the col_count based argument is
    // the first argument now because we're using the xy coordinate system.
    easy.move_xy(col_count - 1, 0);
    easy.print_char('-');

    // Middle of the screen (ish) has a '*'
    easy.move_xy(col_count / 2, row_count / 2);
    easy.print_char('*');

    // Ensure that the user has the latest view of things.
    easy.refresh();

    // Get one input from the user. This is just so that they have a chance to
    // see the message and press a key, otherwise the program would end faster
    // than they could read it.
    easy.get_input();
}
