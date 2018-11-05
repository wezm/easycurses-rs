[![License:0BSD](https://img.shields.io/badge/License-0BSD-brightgreen.svg)](https://opensource.org/licenses/FPL-1.0.0)
[![CratesIO](https://img.shields.io/crates/v/easycurses.svg)](https://crates.io/crates/easycurses)
[![Travis-ci](https://travis-ci.org/Lokathor/easycurses-rs.svg)](https://travis-ci.org/Lokathor/easycurses-rs)
[![Appveyor](https://ci.appveyor.com/api/projects/status/ywxse6ejshe39g1v?svg=true)](https://ci.appveyor.com/project/Lokathor/easycurses-rs)

# EasyCurses

A rust crate to smooth over the pain points of working with curses. Because it's
based on [pancurses](https://github.com/ihalila/pancurses), it works equally
well with on both windows and unix computers.

Examples are available in the `examples/` directory. The files are somewhat
commented, and you can run any of them with `cargo` to see them in action.

```
cargo run --example <fileName>
```

Full API documentation available at [docs.rs/easycurses](https://docs.rs/easycurses)

## Terminal Safety

Normally when you're using curses there's a big danger that your program will
leave the terminal in an unusable state where things don't print properly and
stuff if your program exits on accident and you don't get your chance to call
`endwin` properly. EasyCurses will safely cleanup the terminal and restore it to
a useable state when your program closes via its `Drop` trait. No worries.

The catch is that you do have to only _ever_ have one `EasyCurses` value active
at once. Having two at once would let the initialization and shutdown get out of
balance, and things would go bad. This is tracked with an AtomicBool value
that's flipped on and off as appropriate.

Similarly, if you ever abort the program entirely there's no chance for cleanup,
since an abort is an instant termination of the process. So, just don't ever
compile with `panic=abort`, or use
[exit](https://doc.rust-lang.org/std/process/fn.exit.html), or panic during an
unwind, or anything else like that. At least not while an EasyCurses value is in
scope somewhere within your call stack.

## Stability

I would characterize the library as "largely stable". As laid out somewhere
during the [1.0-level crate
discussions](https://github.com/rust-lang/rust-roadmap/issues/11), a crate can't
rightly call itself 1.0 unless all the things it depends on are themselves 1.0,
so no matter what this crate won't actually go to 1.0 before `pancurses` does.

## License

This project is released into the public domain via [The
Unlicense](https://unlicense.org). You are free to use it however you like with
or without attribution. If you wish to contribute to the project then you must
add your contributions under the same license. 
