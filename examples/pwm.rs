//! Needs the feature `create` enabled

use espc3lib::prelude::*;

fn main() -> anyhow::Result<()> {
	esp_idf_sys::link_patches();

	let mut led = unsafe { RgbLed::pwm() }.unwrap();

	type All = (Red, Green, Blue);

	led.set_duty::<All>(led.get_max_duty::<Red>());

	println!("{}", led.get_duty::<Blue>());

	Ok(())
}
