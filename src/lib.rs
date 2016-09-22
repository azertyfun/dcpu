#![doc(html_root_url = "https://yamakaky.github.io/dcpu/")]

#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate nom;
extern crate num;
#[macro_use]
extern crate glium;

pub mod assembler;
pub mod emulator;
pub mod iterators;
pub mod types;
