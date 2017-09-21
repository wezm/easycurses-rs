
// Prevents a spare console from being created attatched to our program on
// windows, but only if we're running in release mode.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate easycurses;

use easycurses::*;

fn main() {
    // Initialize the system
    let mut easy = EasyCurses::initialize_system().unwrap();

    // don't show the cursor
    easy.set_cursor_visibility(CursorVisibility::Invisible);

    // don't echo the user's input
    easy.set_echo(false);

    // Print this string from the current position. The default cursor position
    // is rc(0,0)
    easy.print("Hello world.");

    // Ensure that the user has the latest view of things.
    easy.refresh();

    // Get one input from the user. This is just so that they have a chance to
    // see the message and press a key, otherwise the program would end faster
    // than they could read it.
    easy.get_input();
}
