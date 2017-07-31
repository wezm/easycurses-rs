
extern crate easycurses;

use easycurses::*;
use std::cmp::{max, min};
use std::iter::repeat;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

fn main() {
    let mut easy = EasyCurses::initialize_system();
    easy.set_cursor_visibility(CursorVisibility::Invisible);
    easy.set_echo(false);
    easy.set_keypad_enabled(true);
    easy.set_input_mode(InputMode::NonBlocking);
    easy.set_scrolling(true);

    let (_, col_count) = easy.get_row_col_count();
    let frame_target_duration = Duration::new(1, 0).checked_div(60).expect(
        "failed when rhs!=0, what?",
    );

    let mut position = 5;
    loop {
        let top_of_loop = Instant::now();
        // Gather/process any pending input
        while let Some(input) = easy.get_input() {
            match input {
                Input::KeyLeft => position = max(0, position - 1),
                Input::KeyRight => position = min(col_count - 1, position + 1),
                _ => (),
            }
        }
        // Compute what we'll display.
        let output = repeat('#').take(position as usize).collect::<String>();

        // Sleep a bit if we need to. This actually sleeps a little longer than
        // just the right time because it doesn't account for the display time
        // we'll use up after the sleep happens. However, curses doesn't really
        // demand perfect animation anyway.
        let elapsed_this_frame = top_of_loop.elapsed();
        if let Some(frame_remaining) = frame_target_duration.checked_sub(elapsed_this_frame) {
            sleep(frame_remaining);
        }

        // Display
        easy.print("\n");
        easy.print(&output);
        easy.refresh();
    }
}
