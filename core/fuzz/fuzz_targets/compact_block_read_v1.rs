#![no_main]
use libfuzzer_sys::fuzz_target;

extern crate gringron_core;

use gringron_core::core::UntrustedCompactBlock;
use gringron_core::ser::{self, DeserializationMode};

fuzz_target!(|data: &[u8]| {
	let mut d = data.clone();
	let _t: Result<UntrustedCompactBlock, ser::Error> =
		ser::deserialize(&mut d, ser::ProtocolVersion(1), DeserializationMode::Full);
});
