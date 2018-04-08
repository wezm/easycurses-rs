extern crate easycurses;

use easycurses::*;
use std::cmp::{max, min};
use std::iter::repeat;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

fn main() {
  // Normal setup
  let mut easy = EasyCurses::initialize_system().unwrap();
  easy.set_cursor_visibility(CursorVisibility::Invisible);
  easy.set_echo(false);
  easy.set_keypad_enabled(true);
  easy.set_input_mode(InputMode::Character);
  easy.set_scrolling(true);

  // We need to know how wide our screen is.
  let (_, mut col_count) = easy.get_row_col_count();

  // Sadly we can't make this const since it has to unwrap and all that, but
  // ideally this could be a const. You could use lazy_static I guess if you
  // really cared, but it's not a huge deal.
  let frame_target_duration = Duration::new(1, 0).checked_div(60).expect("failed when rhs!=0, what?");

  // We start at an arbitrary position.
  let mut position = 5;
  loop {
    let top_of_loop = Instant::now();
    // Gather/process any pending input
    while let Some(input) = easy.get_input() {
      match input {
        Input::KeyLeft => position = max(0, position - 1),
        Input::KeyRight => position = min(col_count - 1, position + 1),
        Input::KeyResize => {
          col_count = easy.get_row_col_count().1;
          position = min(col_count - 1, position);
        }
        other => println!("Unknown: {:?}", other),
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
