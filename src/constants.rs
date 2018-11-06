//! Special constants for things like drawing ascii art.

#[allow(missing_docs)]
pub mod acs {
  use pancurses::*;

  pub fn llcorner() -> chtype {
    ACS_LLCORNER()
  }
  pub fn lrcorner() -> chtype {
    ACS_LRCORNER()
  }
  pub fn ulcorner() -> chtype {
    ACS_ULCORNER()
  }
  pub fn urcorner() -> chtype {
    ACS_URCORNER()
  }
  pub fn btee() -> chtype {
    ACS_BTEE()
  }
  pub fn hline() -> chtype {
    ACS_HLINE()
  }
  pub fn ltee() -> chtype {
    ACS_LTEE()
  }
  pub fn plus() -> chtype {
    ACS_PLUS()
  }
  pub fn rtee() -> chtype {
    ACS_RTEE()
  }
  pub fn ttee() -> chtype {
    ACS_TTEE()
  }
  pub fn vline() -> chtype {
    ACS_VLINE()
  }
  pub fn s1() -> chtype {
    ACS_S1()
  }
  pub fn s9() -> chtype {
    ACS_S9()
  }
  pub fn bullet() -> chtype {
    ACS_BULLET()
  }
  pub fn ckboard() -> chtype {
    ACS_CKBOARD()
  }
  pub fn degree() -> chtype {
    ACS_DEGREE()
  }
  pub fn diamond() -> chtype {
    ACS_DIAMOND()
  }
  pub fn plminus() -> chtype {
    ACS_PLMINUS()
  }
  pub fn block() -> chtype {
    ACS_BLOCK()
  }
  pub fn board() -> chtype {
    ACS_BOARD()
  }
  pub fn darrow() -> chtype {
    ACS_DARROW()
  }
  pub fn lantern() -> chtype {
    ACS_LANTERN()
  }
  pub fn larrow() -> chtype {
    ACS_LARROW()
  }
  pub fn rarrow() -> chtype {
    ACS_RARROW()
  }
  pub fn uarrow() -> chtype {
    ACS_UARROW()
  }
  pub fn s3() -> chtype {
    ACS_S3()
  }
  pub fn s7() -> chtype {
    ACS_S7()
  }
  pub fn gequal() -> chtype {
    ACS_GEQUAL()
  }
  pub fn lequal() -> chtype {
    ACS_LEQUAL()
  }
  pub fn nequal() -> chtype {
    ACS_NEQUAL()
  }
  pub fn pi() -> chtype {
    ACS_PI()
  }
  pub fn sterling() -> chtype {
    ACS_STERLING()
  }
  pub fn bbss() -> chtype {
    ACS_BBSS()
  }
  pub fn bssb() -> chtype {
    ACS_BSSB()
  }
  pub fn sbbs() -> chtype {
    ACS_SBBS()
  }
  pub fn sbss() -> chtype {
    ACS_SBSS()
  }
  pub fn ssbb() -> chtype {
    ACS_SSBB()
  }
  pub fn ssbs() -> chtype {
    ACS_SSBS()
  }
  pub fn sssb() -> chtype {
    ACS_SSSB()
  }
  pub fn bsbs() -> chtype {
    ACS_BSBS()
  }
  pub fn bsss() -> chtype {
    ACS_BSSS()
  }
  pub fn sbsb() -> chtype {
    ACS_SBSB()
  }
  pub fn ssss() -> chtype {
    ACS_SSSS()
  }
}
