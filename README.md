[![Crates](https://img.shields.io/crates/v/easycurses.svg)](https://crates.io/crates/easycurses)

# EasyCurses

A rust crate to smooth over the pain points of working with curses. Because it's
based on [pancurses](https://github.com/ihalila/pancurses), it works equally
well with on both windows and unix computers.

Examples are available in the `examples/` directory. The files are throughly
commented, and you can run any of them with `cargo` to see them in action.

```
cargo run --example <fileName>
```

## [Documentation](https://docs.rs/easycurses)

## Terminal Safety

Normally when you're using curses there's a big danger that your program will
leave the terminal in an unsable state where things don't print properly and
stuff if your program exits on accident and you don't get your chance to call
`endwin` properly. EasyCurses will safely cleanup the terminal and restore it to
a useable state when your program closes via its `Drop` trait. No worries.

The catch is that you do have to only _ever_ have one `EasyCurses` value active
at once. Having two at once would let the initialization and shutdown get out of
balance, and things would go bad. Currently there's nothing enforcing this at
all, but in the future there might be a way to enforce this without burdening
the users of the library. Note that it is safe to drop `EasyCurses` entirely
(shutting down curses in the process) and then make a new one (starting a fresh
new curses session).

Similarly, if you ever abort the program entirely there's no chance for cleanup,
since an abort is an instant termination of the process. So, just don't ever
compile with `panic=abort`, or use
[exit](https://doc.rust-lang.org/std/process/fn.exit.html), or panic during an
unwind, or anything else like that. At least not while an EasyCurses value is in
scope somewhere within your call stack.

## Stability

I would characterize the library as largely stable. It's missing at least one
feature that I'd like to see added (insert_char) but that's because pancurses
itself lacks that at the moment.

As laid out somewhere during the [1.0-level crate
discussions](https://github.com/rust-lang/rust-roadmap/issues/11), a crate can't
rightly call itself 1.0 unless all the things it depends on are themselves 1.0,
so no matter what this crate won't actaully go to 1.0 before `pancurses` does.

## License

This project uses Rust's standard Apache/MIT dual-license scheme. In other
words, you can use it under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE.txt) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT.txt) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
