//! # Description
//! 
//! This binary is a small executable that tests the basic functionality of 
//! the `throrgan` crate. This binary is not intended to be published, and if
//! you are reading this, it was likely published accidentally.
use throrgan;

fn main() {
    throrgan::compile("foo.txt", "").unwrap();
}