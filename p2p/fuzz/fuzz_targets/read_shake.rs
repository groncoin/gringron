#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate gringron_core;
extern crate gringron_p2p;

use gringron_core::ser;
use gringron_p2p::msg::Shake;

fuzz_target!(|data: &[u8]| {
	let mut d = data.clone();
	let _t: Result<Shake, ser::Error> = ser::deserialize(&mut d);
});
