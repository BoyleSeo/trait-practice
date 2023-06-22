mod js_map;
mod other;
mod terminal;

#[cfg(feature = "js_map")]
use js_map::*;
#[cfg(not(any(feature = "js_map", feature = "terminal_api")))]
use other::*;
#[cfg(feature = "terminal_api")]
use terminal::*;

fn main() {
    println!("\n[VERSION 1]");
    version_1::test();
    println!("\n[VERSION 2]");
    version_2::test();
}
