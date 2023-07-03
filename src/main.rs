#[cfg(feature = "js_map")]
#[path = "js_map/mod.rs"]
mod feature;

#[cfg(not(any(feature = "js_map", feature = "terminal_api")))]
#[path = "other.rs"]
mod feature;
#[cfg(feature = "terminal_api")]
#[path = "terminal/mod.rs"]
mod feature;

use feature::*;
fn main() {
    println!("\n[VERSION 1]");
    version_1::test();
    println!("\n[VERSION 2]");
    version_2::test();
}
