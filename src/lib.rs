#![feature(concat_idents)]

pub mod accessor;
pub mod prelude;

#[cfg(feature = "create")]
pub mod create;

pub struct RgbLed<RED, GREEN, BLUE> {
	red: RED,
	green: GREEN,
	blue: BLUE,
}
