#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate easycurses;

use easycurses::*;
use easycurses::constants::acs;
use easycurses::Color::*;

fn main() {
    let mut easy = EasyCurses::initialize_system().unwrap();

    easy.set_echo(false);

    easy.set_color_pair(colorpair!(Green on Black));

    easy.move_rc(0, 0);

    easy.insert_char(acs::llcorner());
    easy.insert_char(acs::lrcorner());
    easy.insert_char(acs::ulcorner());
    easy.insert_char(acs::urcorner());
    easy.insert_char(acs::btee());
    easy.insert_char(acs::hline());
    easy.insert_char(acs::ltee());
    easy.insert_char(acs::plus());
    easy.insert_char(acs::rtee());
    easy.insert_char(acs::ttee());
    easy.insert_char(acs::vline());
    easy.insert_char(acs::s1());
    easy.insert_char(acs::s9());

    easy.move_rc(1, 0);

    easy.insert_char(acs::bullet());
    easy.insert_char(acs::ckboard());
    easy.insert_char(acs::degree());
    easy.insert_char(acs::diamond());
    easy.insert_char(acs::plminus());
    easy.insert_char(acs::block());
    easy.insert_char(acs::board());
    easy.insert_char(acs::darrow());
    easy.insert_char(acs::lantern());
    easy.insert_char(acs::larrow());
    easy.insert_char(acs::rarrow());
    easy.insert_char(acs::uarrow());
    easy.insert_char(acs::s3());

    easy.move_rc(2, 0);

    easy.insert_char(acs::s7());
    easy.insert_char(acs::gequal());
    easy.insert_char(acs::lequal());
    easy.insert_char(acs::nequal());
    easy.insert_char(acs::pi());
    easy.insert_char(acs::sterling());
    easy.insert_char(acs::bbss());
    easy.insert_char(acs::bssb());
    easy.insert_char(acs::sbbs());
    easy.insert_char(acs::sbss());
    easy.insert_char(acs::ssbb());
    easy.insert_char(acs::ssbs());
    easy.insert_char(acs::sssb());
    easy.insert_char(acs::bsbs());
    easy.insert_char(acs::bsss());
    easy.insert_char(acs::sbsb());
    easy.insert_char(acs::ssss());

    easy.refresh();

    easy.get_input();
}
