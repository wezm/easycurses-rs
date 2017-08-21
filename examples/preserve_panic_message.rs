
extern crate easycurses;

use easycurses::*;

fn main() {
    // We wrap all our use of curses with this function.
    preserve_panic_message(|easy| {
        // In here we get an initialized EasyCurses handle and then proceed to
        // use it exactly like we normally would use it.
        easy.set_cursor_visibility(CursorVisibility::Invisible);
        easy.set_echo(false);
        easy.print("Hello world.");
        easy.refresh();
        easy.get_input();
        panic!("oh no");
    }).unwrap_or_else(|e| match e {
        // This block only runs if there was an error. We might or might not
        // have been able to recover an error message. You technically can pass
        // any value into a panic, but we only get an error message if the panic
        // value was a `String` or `&str`.
        Some(errmsg) => println!("Error Occurred: {}", errmsg),
        None => println!("There was an error, but no error message."),
    });
}
