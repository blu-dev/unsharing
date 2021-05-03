#![feature(proc_macro_hygiene)]
#![feature(ptr_offset_from)]

mod loaded;
mod offsets;
mod utils;
mod vector;

#[skyline::main(name = "unsharing")]
pub fn main() {
    offsets::search_offsets();
}