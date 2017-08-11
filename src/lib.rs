
#![warn(missing_docs)]
#![allow(dead_code)]
#![deny(missing_debug_implementations)]

//! This is a crate that allows one to easily use a basic form of curses. It is
//! based upon [pancurses](https://docs.rs/crate/pancurses/0.8.0) and so it's
//! cross platform between windows and unix. It exposes a simplified view of
//! curses functionality where there's just one Window and all of your actions
//! are called upon a single struct type, `EasyCurses`. This ensures that curses
//! functions are only called while curses is initialized, and also that curses
//! is always cleaned up at the end (via `Drop`).
//!
//! You should _never_ make a second `EasyCurses` value without having ensured
//! that the first one is already dropped. Initialization and shutdown of curses
//! will get out of balance and your terminal will probably be left in a very
//! unusable state.
//!
//! Similarly, the library can only perform proper automatic cleanup if Rust is
//! allowed to run the `Drop` implementation. This happens normally, and during
//! an unwinding pancic, but if you ever abort the program (either because you
//! compiled with `panic=abort` or because you panic during an unwind) you lose
//! the cleanup safety. So, don't do that.

extern crate pancurses;

pub use pancurses::Input;

use std::panic::*;
use std::iter::Iterator;

/// The three options you can pass to [`EasyCurses::set_cursor_visibility`].
///
/// Note that not all terminals support all visibility modes.
///
/// [`EasyCurses::set_cursor_visibility`]: struct.EasyCurses.html#method.set_cursor_visibility
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CursorVisibility {
    /// Makes the cursor invisible. Supported on most terminals.
    Invisible,
    /// Makes the cursor visible in the normal way. The Default.
    Visible,
    /// Makes the cursor "highly" visible in some way. Not supported on all terminals.
    HighlyVisible,
}

impl Default for CursorVisibility {
    fn default() -> Self {
        CursorVisibility::Visible
    }
}

/// The curses color constants.
///
/// Curses supports eight different colors. Each character cell has one "color
/// pair" set which is a foreground and background pairing. Note that a cell can
/// also be "bold", which might display as differnet colors on some terminals.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    #[allow(missing_docs)]
    Black,
    #[allow(missing_docs)]
    Red,
    #[allow(missing_docs)]
    Green,
    #[allow(missing_docs)]
    Yellow,
    #[allow(missing_docs)]
    Blue,
    #[allow(missing_docs)]
    Magenta,
    #[allow(missing_docs)]
    Cyan,
    #[allow(missing_docs)]
    White,
}

type ColorIter = std::iter::Cloned<std::slice::Iter<'static, Color>>;

impl Color {
    /// Provides a handy Iterator over all of the Color values.
    pub fn color_iterator() -> ColorIter {
        use Color::*;
        #[allow(non_upper_case_globals)]
        static colors: &[Color] = &[Black, Red, Green, Yellow, Blue, Magenta, Cyan, White];
        colors.iter().cloned()
    }
}

/// Converts a `Color` to the `i16` associated with it.
fn color_to_i16(color: Color) -> i16 {
    use Color::*;
    match color {
        Black => 0,
        Red => 1,
        Green => 2,
        Yellow => 3,
        Blue => 4,
        Magenta => 5,
        Cyan => 6,
        White => 7,
    }
}

/// Converts an `i16` to the `Color` associated with it. Fails if the input is
/// outside the range 0 to 7 (inclusive).
fn i16_to_color(val: i16) -> Option<Color> {
    use Color::*;
    match val {
        0 => Some(Black),
        1 => Some(Red),
        2 => Some(Green),
        3 => Some(Yellow),
        4 => Some(Blue),
        5 => Some(Magenta),
        6 => Some(Cyan),
        7 => Some(White),
        _ => None,
    }
}

#[cfg(test)]
mod color_tests {
    use super::*;

    #[test]
    fn test_color_i32_conversion_identity() {
        use Color::*;
        let colors = [Black, Red, Green, Yellow, Blue, Magenta, Cyan, White];
        for &color in colors.iter() {
            if i16_to_color(color_to_i16(color)).unwrap() != color {
                panic!(color);
            }
        }
    }

    #[test]
    fn test_color_i32_matches_color_constants() {
        use Color::*;
        assert!(color_to_i16(Black) == pancurses::COLOR_BLACK);
        assert!(color_to_i16(Red) == pancurses::COLOR_RED);
        assert!(color_to_i16(Green) == pancurses::COLOR_GREEN);
        assert!(color_to_i16(Yellow) == pancurses::COLOR_YELLOW);
        assert!(color_to_i16(Blue) == pancurses::COLOR_BLUE);
        assert!(color_to_i16(Magenta) == pancurses::COLOR_MAGENTA);
        assert!(color_to_i16(Cyan) == pancurses::COLOR_CYAN);
        assert!(color_to_i16(White) == pancurses::COLOR_WHITE);
    }
}

/// A color pair for a character cell on the screen.
///
/// Use them with [`EasyCurses::set_color_pair`].
///
/// [`EasyCurses::set_color_pair`]: struct.EasyCurses.html#method.set_color_pair
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ColorPair(i16);

impl ColorPair {
    /// Creates a new `ColorPair` given a foreground and background.
    pub fn new(fg: Color, bg: Color) -> Self {
        let fgi = color_to_i16(fg);
        let bgi = color_to_i16(bg);
        ColorPair(fgbg_pairid(fgi, bgi))
    }
}

impl Default for ColorPair {
    /// The "default" color pair is White text on a Black background.
    ///
    /// ```
    /// use easycurses::{Color,ColorPair};
    /// assert_eq!(ColorPair::default(), ColorPair::new(Color::White,Color::Black));
    /// ```
    fn default() -> Self {
        Self::new(Color::White, Color::Black)
    }
}

/// The "low level" conversion using i16 values. Color pair 0 is white on
/// black but we can't assign to it. Technically we're only assured to have
/// color pairs 0 through 63 available, but you usually get more so we're
/// taking a gamble that there's at least one additional bit available. The
/// alternative is a somewhat complicated conversion scheme where we special
/// case White/Black to be 0, then other things start ascending above that,
/// until we hit where White/Black should be and start subtracting one from
/// everything to keep it within spec. I don't wanna do that if I don't
/// really have to.
fn fgbg_pairid(fg: i16, bg: i16) -> i16 {
    1 + (8 * fg + bg)
}

/// Converts a `pancurses::OK` value into `true`, and all other values into
/// `false`.
fn to_bool(curses_bool: i32) -> bool {
    if curses_bool == pancurses::OK {
        true
    } else {
        false
    }
}

/// Wraps the use of curses with `catch_unwind` to preserve panic info.
///
/// Normally, if your program panics while in curses mode the panic message
/// prints immediately and then is destroyed before you can see it by the
/// automatic cleanup of curses mode. Instead, this runs the function you pass
/// it within `catch_unwind` and when there's a panic it catches the panic value
/// and attempts to downcast it into a `String` you can print out or log or
/// whatever you like. Since a panic value can be anything at all this won't
/// always succeed, thus the `Option` wrapper on the `Err` case. Regardless of
/// what of `Result` you get back, curses mode will be fully cleaned up and shut
/// down by the time this function returns.
///
/// Note that you *don't* have to use this if you just want your terminal
/// restored to normal when your progam panics while in curses mode. That is
/// handled automatically by the `Drop` implementation of `EasyCurses`. You only
/// need to use this if you care about the panic message itself.
pub fn preserve_panic_message<F: FnOnce(&mut EasyCurses) -> R + UnwindSafe, R>(
    user_function: F,
) -> Result<R, Option<String>> {
    let result = catch_unwind(|| {
        let mut easy = EasyCurses::initialize_system();
        user_function(&mut easy)
    });
    result.map_err(|e| match e.downcast_ref::<&str>() {
        Some(andstr) => Some(andstr.to_string()),
        None => {
            match e.downcast_ref::<String>() {
                Some(string) => Some(string.to_string()),
                None => None,
            }
        }
    })
}

/// This is a handle to all your fun curses functionality.
///
/// `EasyCurses` will automatically restore the terminal when you drop it, so
/// you don't need to worry about any manual cleanup. Automatic cleanup will
/// happen even if your program panics and unwinds, but it **will not** happen
/// if your program panics and aborts (obviously). So, don't abort the program
/// while curses is active, or your terminal session will just be ruined.
///
/// Except in the case of [`is_color_terminal`], all `EasyCurses` methods that
/// return a `bool` use it to indicate if the requested operation was successful
/// or not.
///
/// [`is_color_terminal`]: #method.is_color_terminal
#[derive(Debug)]
pub struct EasyCurses {
    /// This is the inner pancurses `Window` that the `EasyCurses` type wraps
    /// over.
    ///
    /// This is only intended to be used as a last resort, if you really want to
    /// call something that's not here. Under normal circumstances you shouldn't
    /// need to touch this field at all. It's not "unsafe" to use in the
    /// rust/memory sense, but if you access this field and then cause a bug in
    /// `EasyCurses`, that's your fault.
    pub win: pancurses::Window,
    color_support: bool,
}

impl Drop for EasyCurses {
    /// Dropping EasyCurses causes the
    /// [endwin](http://pubs.opengroup.org/onlinepubs/7908799/xcurses/endwin.html)
    /// curses function to be called.
    fn drop(&mut self) {
        pancurses::endwin();
    }
}

impl EasyCurses {
    /// Initializes the curses system so that you can begin using curses. This
    /// isn't called "new" because you shouldn't be making more than one
    /// EasyCurses value at the same time ever.
    ///
    /// If the terminal supports colors, they are automatcially activated and
    /// color pairs are initialized for all color foreground and background
    /// combinations.
    ///
    /// # Panics
    ///
    /// Since this uses the
    /// [initscr](http://pubs.opengroup.org/onlinepubs/7908799/xcurses/initscr.html)
    /// curses function, any error during initialization will generally cause
    /// the program to print an error message to stdout and then immediately
    /// exit. C libs are silly like that. Your terminal _will_ be left in a
    /// usable state, but anything else in your program that's not abort-safe is
    /// probably not fullly safe with this. It is expected that you deal with
    /// that by calling this just once at the start of your program, before
    /// you've started anything that's not abort-safe.
    pub fn initialize_system() -> Self {
        let w = pancurses::initscr();
        let color_support = if pancurses::has_colors() {
            to_bool(pancurses::start_color())
        } else {
            false
        };
        if color_support {
            let color_count = pancurses::COLORS();
            let pair_count = pancurses::COLOR_PAIRS();
            for fg in Color::color_iterator() {
                for bg in Color::color_iterator() {
                    let fgi = color_to_i16(fg);
                    let bgi = color_to_i16(bg);
                    let pair_id = fgbg_pairid(fgi, bgi);
                    assert!(fgi <= color_count as i16);
                    assert!(bgi <= color_count as i16);
                    assert!(pair_id <= pair_count as i16);
                    pancurses::init_pair(pair_id, fgi, bgi);
                }
            }
        }
        EasyCurses {
            win: w,
            color_support: color_support,
        }
    }

    /// On Win32 systems this allows you to set the title of the PDcurses
    /// window. On other systems this does nothing at all.
    pub fn set_title_win32(&mut self, title: &str) {
        pancurses::set_title(title);
    }

    /// Attempts to assign a new cursor visibility. If this is successful you
    /// get a `Some` back with the old setting inside. If this fails you get a
    /// `None` back. For more info see
    /// [curs_set](http://pubs.opengroup.org/onlinepubs/7908799/xcurses/curs_set.html)
    pub fn set_cursor_visibility(&mut self, vis: CursorVisibility) -> Option<CursorVisibility> {
        use CursorVisibility::*;
        let result = pancurses::curs_set(match vis {
            Invisible => 0,
            Visible => 1,
            HighlyVisible => 2,
        });
        match result {
            0 => Some(Invisible),
            1 => Some(Visible),
            2 => Some(HighlyVisible),
            _ => None,
        }
    }

    /// In character break mode (cbreak), characters typed by the user are made
    /// available immediately, and erase/kill/backspace character processing is
    /// not performed. When this mode is off (nocbreak) user input is not
    /// available to the application until a newline has been typed. The default
    /// mode is not specified (but happens to often be cbreak).
    ///
    /// See also the [Input
    /// Mode](http://pubs.opengroup.org/onlinepubs/7908799/xcurses/intov.html#tag_001_005_002)
    /// section of the curses documentation.
    pub fn set_character_break(&mut self, cbreak: bool) -> bool {
        if cbreak {
            to_bool(pancurses::cbreak())
        } else {
            to_bool(pancurses::nocbreak())
        }
    }

    /// Enables special key processing from buttons such as the keypad and arrow
    /// keys. This defaults to `false`. You probably want to set it to `true`.
    /// If it's not on and the user presses a special key then get_key will
    /// return will do nothing or give `ERR`.
    pub fn set_keypad_enabled(&mut self, use_keypad: bool) -> bool {
        to_bool(self.win.keypad(use_keypad))
    }

    /// Enables or disables the automatic echoing of input into the window as
    /// the user types. Default to on, but you probably want it to be off most
    /// of the time.
    pub fn set_echo(&mut self, echoing: bool) -> bool {
        to_bool(if echoing {
            pancurses::echo()
        } else {
            pancurses::noecho()
        })
    }

    // TODO: pancurses::resize_term?

    /// Checks if the current terminal supports the use of colors.
    pub fn is_color_terminal(&mut self) -> bool {
        self.color_support
    }

    /// Sets the current color pair of the window. Output at any location will
    /// use this pair until a new pair is set. Does nothing if the terminal does
    /// not support colors in the first place.
    pub fn set_color_pair(&mut self, pair: ColorPair) {
        if self.color_support {
            self.win.color_set(pair.0);
        }
    }

    /// Enables or disables bold text for all future input.
    pub fn set_bold(&mut self, bold_on: bool) -> bool {
        to_bool(if bold_on {
            self.win.attron(pancurses::Attribute::Bold)
        } else {
            self.win.attroff(pancurses::Attribute::Bold)
        })
    }

    /// Enables or disables unerlined text for all future input.
    pub fn set_underline(&mut self, underline_on: bool) -> bool {
        to_bool(if underline_on {
            self.win.attron(pancurses::Attribute::Underline)
        } else {
            self.win.attroff(pancurses::Attribute::Underline)
        })
    }

    /// Returns the number of rows and columns available in the window.
    pub fn get_row_col_count(&mut self) -> (i32, i32) {
        self.win.get_max_yx()
    }

    /// Moves the virtual cursor to the row and column specified, relative to
    /// the top left ("notepad" space). Does not move the terminal's dispayed
    /// cursor (if any) until `refresh` is also called. Out of bounds locations
    /// cause this command to be ingored.
    pub fn move_rc(&mut self, row: i32, col: i32) -> bool {
        to_bool(self.win.mv(row, col))
    }

    /// Moves the virtual cursor to the x and y specified, relative to the
    /// bottom left ("cartesian" space). Does not move the terminal's displayed
    /// cursor (if any) until `refresh` is also called. Out of bounds locations
    /// cause this command to be ingored.
    pub fn move_xy(&mut self, x: i32, y: i32) -> bool {
        let row_count = self.win.get_max_y();
        to_bool(self.win.mv(row_count - (y + 1), x))
    }

    /// When scrolling is enabled, any attempt to move off the bottom margin
    /// will cause lines within the scrolling region to scroll up one line. If a
    /// scrolling region is set but scolling is not enabled then attempts to go
    /// off the bottom will just print nothing instead. Use `set_scroll_region`
    /// to control the size of the scrolling region.
    pub fn set_scrolling(&mut self, scrolling: bool) -> bool {
        to_bool(self.win.scrollok(scrolling))
    }

    /// Sets the top and bottom margins of the scrolling region.
    pub fn set_scroll_region(&mut self, top: i32, bottom: i32) -> bool {
        to_bool(self.win.setscrreg(top, bottom))
    }

    /// Prints the given string into the window.
    pub fn print(&mut self, string: &str) -> bool {
        to_bool(self.win.printw(string))
    }

    /// Prints the given character into the window.
    pub fn print_char(&mut self, character: char) -> bool {
        to_bool(self.win.addch(character))
    }

    /// Deletes the character under the cursor. Characters after it on same the
    /// line are pulled left one position and the final character cell is left
    /// blank. The cursor position does not move.
    pub fn delete_char(&mut self) -> bool {
        to_bool(self.win.delch())
    }

    /// Deletes the line under the cursor. Lines below are moved up one line and
    /// the final line is left blank. The cursor position does not move.
    pub fn delete_line(&mut self) -> bool {
        to_bool(self.win.deleteln())
    }

    /// Clears the entire screen.
    pub fn clear(&mut self) -> bool {
        to_bool(self.win.clear())
    }

    /// Refreshes the window's appearance on the screen. With some
    /// implementations you don't need to call this, the screen will refresh
    /// itself on its own. However, for portability, you should call this at the
    /// end of each draw cycle.
    pub fn refresh(&mut self) -> bool {
        to_bool(self.win.refresh())
    }

    /// Plays an audible beep if possible, if not the screen is flashed. If
    /// neither is available then nothing happens.
    pub fn beep(&mut self) {
        pancurses::beep();
    }

    /// Flashes the screen if possible, if not an audible beep is played. If
    /// neither is available then nothing happens.
    pub fn flash(&mut self) {
        pancurses::flash();
    }

    /// This controls if `get_input` is blocking or not. The default mode is `Blocking`.
    ///
    /// See also: The
    /// [notimeout](http://pubs.opengroup.org/onlinepubs/7908799/xcurses/notimeout.html)
    /// curses function.
    pub fn set_input_mode(&mut self, mode: InputMode) {
        use InputMode::*;
        use std::cmp::max;
        match mode {
            Blocking => self.win.timeout(-1),
            NonBlocking => self.win.timeout(0),
            TimeLimit(time) => self.win.timeout(max(time, 1)),
        };
    }

    /// Gets an `Input` from the curses input buffer. Depending on the `timeout` setting that y
    pub fn get_input(&mut self) -> Option<pancurses::Input> {
        self.win.getch()
    }

    /// Discards all type-ahead that has been input by the user but not yet read
    /// by the program.
    pub fn flush_input(&mut self) {
        pancurses::flushinp();
    }

    /// Pushes an `Input` value into the input stack so that it will be returned
    /// by the next call to `get_input`. The return value is if the operation
    /// was successful.
    pub fn un_get_input(&mut self, input: &pancurses::Input) -> bool {
        to_bool(self.win.ungetch(input))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// The input modes allowed.
///
/// Affects how [`EasyCurses::get_input`] works. Set a mode with
/// [`EasyCurses::set_input_mode`].
///
/// [`EasyCurses::get_input`]: struct.EasyCurses.html#method.get_input
///
/// [`EasyCurses::set_input_mode`]: struct.EasyCurses.html#method.set_input_mode
pub enum InputMode {
    /// `get_input` will block indefinitely. This is the default.
    Blocking,
    /// `get_input` will return immediately. If no input is in the queue you get
    /// a `None` value back.
    NonBlocking,
    /// `get_input` will wait up to the number of milliseconds specified before
    /// returning `None`. If any value less than 1 is given, it uses 1 instead.
    TimeLimit(i32),
}

impl Default for InputMode {
    /// The default input mode is Blocking.
    fn default() -> Self {
        InputMode::Blocking
    }
}
