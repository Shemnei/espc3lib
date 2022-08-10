use std::sync::Arc;

use esp_idf_hal::gpio::{Gpio3, Gpio4, Gpio5, InputOutput, Output, Unknown};
use esp_idf_hal::ledc::config::TimerConfig;
use esp_idf_hal::ledc::{
	Channel, Timer, CHANNEL0, CHANNEL1, CHANNEL2, TIMER0,
};
use esp_idf_hal::units::*;
use esp_idf_sys::EspError;

use crate::RgbLed;

pub type Esp32C3RgbLedIO =
	RgbLed<Gpio3<InputOutput>, Gpio4<InputOutput>, Gpio5<InputOutput>>;
pub type Esp32C3RgbLedO = RgbLed<Gpio3<Output>, Gpio4<Output>, Gpio5<Output>>;

pub type ChannelRed =
	Channel<CHANNEL0, TIMER0, Arc<Timer<TIMER0>>, Gpio3<Unknown>>;
pub type ChannelGreen =
	Channel<CHANNEL1, TIMER0, Arc<Timer<TIMER0>>, Gpio4<Unknown>>;
pub type ChannelBlue =
	Channel<CHANNEL2, TIMER0, Arc<Timer<TIMER0>>, Gpio5<Unknown>>;
pub type Esp32C3RgbLedPWM = RgbLed<ChannelRed, ChannelGreen, ChannelBlue>;

impl Esp32C3RgbLedIO {
	/// # Safety
	/// Caller must ensure that the pins `Gpio3`, `Gpio4` and `Gpio5` are not
	/// used anywhere else in the calling code.
	pub unsafe fn digital_inout() -> Result<Self, EspError> {
		let pin_red = Gpio3::<Unknown>::new().into_input_output()?;
		let pin_green = Gpio4::<Unknown>::new().into_input_output()?;
		let pin_blue = Gpio5::<Unknown>::new().into_input_output()?;

		Ok(Self { red: pin_red, green: pin_green, blue: pin_blue })
	}
}

impl Esp32C3RgbLedO {
	/// # Safety
	/// Caller must ensure that the pins `Gpio3`, `Gpio4` and `Gpio5` are not
	/// used anywhere else in the calling code.
	pub unsafe fn digital_out() -> Result<Self, EspError> {
		let pin_red = Gpio3::<Unknown>::new().into_output()?;
		let pin_green = Gpio4::<Unknown>::new().into_output()?;
		let pin_blue = Gpio5::<Unknown>::new().into_output()?;

		Ok(Self { red: pin_red, green: pin_green, blue: pin_blue })
	}
}

impl Esp32C3RgbLedPWM {
	/// # Safety
	/// Caller must ensure that the pins `Gpio3`, `Gpio4` and `Gpio5` are not
	/// used anywhere else in the calling code.
	pub unsafe fn pwm() -> Result<Self, EspError> {
		let config = TimerConfig::default().frequency(25.kHz().into());
		let timer = Arc::new(Timer::new(TIMER0::new(), &config)?);

		let channel_red = Channel::new(
			CHANNEL0::new(),
			timer.clone(),
			Gpio3::<Unknown>::new(),
		)?;
		let channel_green = Channel::new(
			CHANNEL1::new(),
			timer.clone(),
			Gpio4::<Unknown>::new(),
		)?;
		let channel_blue =
			Channel::new(CHANNEL2::new(), timer, Gpio5::<Unknown>::new())?;

		Ok(Self { red: channel_red, green: channel_green, blue: channel_blue })
	}
}
