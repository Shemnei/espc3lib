//! Needs the feature `create` enabled

use embedded_hal::digital::v2::PinState;
use espc3lib::prelude::*;

fn main() -> anyhow::Result<()> {
	esp_idf_sys::link_patches();

	let mut led = unsafe { RgbLed::digital_out() }.unwrap();

	led.set_low::<Red>()?;

	// Any tuple combination of `Red`, `Blue`, `Green` works
	led.set_high::<(Red, Blue)>()?;
	led.set_high::<(Blue, Red)>()?;

	led.set_state::<Green>(false)?;
	led.set_state::<Green>(PinState::High)?;

	type All = (Red, Green, Blue);
	led.set_low::<All>().unwrap();

	type SpecialCombination = (Red, Blue);
	led.toggle::<SpecialCombination>().unwrap();

	// Only returns `true` if every led is low
	led.is_set_low::<(Red, Green)>().unwrap();
	// Only returns `true` if every led is high
	led.is_set_high::<(Green, Red)>().unwrap();

	Ok(())
}
