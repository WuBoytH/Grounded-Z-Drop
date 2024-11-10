#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    ambiguous_glob_reexports,
    unused_macros
)]

pub mod func_links;

mod common;
mod items;

#[skyline::main(name = "ground_z_drop_all")]
pub fn main() {
    common::install();
    items::install();
}