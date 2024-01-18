#![no_main]
use libfuzzer_sys::fuzz_target;

extern crate gringron_core;

use gringron_core::core::UntrustedBlock;
use gringron_core::ser::{self, DeserializationMode};

fuzz_target!(|data: &[u8]| {
	let mut d = data.clone();
	let _t: Result<UntrustedBlock, ser::Error> =
		ser::deserialize(&mut d, ser::ProtocolVersion(2), DeserializationMode::Full);
});
