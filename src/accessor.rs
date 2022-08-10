use embedded_hal::digital::v2::{OutputPin, PinState};
#[cfg(feature = "create")]
use embedded_hal::digital::v2::{StatefulOutputPin, ToggleableOutputPin};
use embedded_hal::PwmPin;

use crate::RgbLed;

pub trait OutputAcessor<RED, GREEN, BLUE> {
	type Error;

	fn set_state<S: Into<PinState>>(
		led: &mut RgbLed<RED, GREEN, BLUE>,
		state: S,
	) -> Result<(), Self::Error>;
	fn set_low(led: &mut RgbLed<RED, GREEN, BLUE>) -> Result<(), Self::Error>;
	fn set_high(led: &mut RgbLed<RED, GREEN, BLUE>)
		-> Result<(), Self::Error>;
}

impl<RED, GREEN, BLUE> RgbLed<RED, GREEN, BLUE> {
	pub fn set_state<A>(
		&mut self,
		state: impl Into<PinState>,
	) -> Result<(), A::Error>
	where
		A: OutputAcessor<RED, GREEN, BLUE>,
	{
		A::set_state(self, state)
	}

	pub fn set_low<A>(&mut self) -> Result<(), A::Error>
	where
		A: OutputAcessor<RED, GREEN, BLUE>,
	{
		A::set_low(self)
	}

	pub fn set_high<A>(&mut self) -> Result<(), A::Error>
	where
		A: OutputAcessor<RED, GREEN, BLUE>,
	{
		A::set_high(self)
	}
}

#[cfg(feature = "create")]
pub trait StatefulOutputAcessor<RED, GREEN, BLUE>:
	OutputAcessor<RED, GREEN, BLUE>
{
	fn is_set_low(led: &RgbLed<RED, GREEN, BLUE>)
		-> Result<bool, Self::Error>;
	fn is_set_high(
		led: &RgbLed<RED, GREEN, BLUE>,
	) -> Result<bool, Self::Error>;
}

#[cfg(feature = "create")]
impl<RED, GREEN, BLUE> RgbLed<RED, GREEN, BLUE> {
	pub fn is_set_low<A>(&self) -> Result<bool, A::Error>
	where
		A: StatefulOutputAcessor<RED, GREEN, BLUE>,
	{
		A::is_set_low(self)
	}

	pub fn is_set_high<A>(&self) -> Result<bool, A::Error>
	where
		A: StatefulOutputAcessor<RED, GREEN, BLUE>,
	{
		A::is_set_high(self)
	}
}

#[cfg(feature = "create")]
pub trait ToggleableOutputAcessor<RED, GREEN, BLUE> {
	type Error;

	fn toggle(led: &mut RgbLed<RED, GREEN, BLUE>) -> Result<(), Self::Error>;
}

#[cfg(feature = "create")]
impl<RED, GREEN, BLUE> RgbLed<RED, GREEN, BLUE> {
	pub fn toggle<A>(&mut self) -> Result<(), A::Error>
	where
		A: ToggleableOutputAcessor<RED, GREEN, BLUE>,
	{
		A::toggle(self)
	}
}

pub trait PwmAccessor<RED, GREEN, BLUE> {
	type Duty;

	fn disable(led: &mut RgbLed<RED, GREEN, BLUE>);
	fn enable(led: &mut RgbLed<RED, GREEN, BLUE>);
	fn get_duty(led: &RgbLed<RED, GREEN, BLUE>) -> Self::Duty;
	fn get_max_duty(led: &RgbLed<RED, GREEN, BLUE>) -> Self::Duty;
	fn set_duty(led: &mut RgbLed<RED, GREEN, BLUE>, duty: Self::Duty);
}

impl<RED, GREEN, BLUE> RgbLed<RED, GREEN, BLUE> {
	pub fn disable<A>(&mut self)
	where
		A: PwmAccessor<RED, GREEN, BLUE>,
	{
		A::disable(self)
	}

	pub fn enable<A>(&mut self)
	where
		A: PwmAccessor<RED, GREEN, BLUE>,
	{
		A::enable(self)
	}

	pub fn get_duty<A>(&self) -> A::Duty
	where
		A: PwmAccessor<RED, GREEN, BLUE>,
	{
		A::get_duty(self)
	}

	pub fn get_max_duty<A>(&self) -> A::Duty
	where
		A: PwmAccessor<RED, GREEN, BLUE>,
	{
		A::get_max_duty(self)
	}

	pub fn set_duty<A>(&mut self, duty: A::Duty)
	where
		A: PwmAccessor<RED, GREEN, BLUE>,
	{
		A::set_duty(self, duty)
	}
}

pub trait PwmAccessorExt<RED, GREEN, BLUE>:
	PwmAccessor<RED, GREEN, BLUE>
{
	fn set_duty_frac(led: &mut RgbLed<RED, GREEN, BLUE>, fraction: f32);
}

impl<RED, GREEN, BLUE> RgbLed<RED, GREEN, BLUE> {
	pub fn set_duty_frac<A>(&mut self, fraction: f32)
	where
		A: PwmAccessorExt<RED, GREEN, BLUE>,
	{
		A::set_duty_frac(self, fraction)
	}
}

pub trait PwmAccessorRgbExt<RED, GREEN, BLUE>:
	PwmAccessorExt<RED, GREEN, BLUE>
{
	fn set_rgb(led: &mut RgbLed<RED, GREEN, BLUE>, r: f32, g: f32, b: f32);

	fn set_rgb_u8(led: &mut RgbLed<RED, GREEN, BLUE>, r: u8, g: u8, b: u8) {
		Self::set_rgb(
			led,
			r as f32 / u8::MAX as f32,
			g as f32 / u8::MAX as f32,
			b as f32 / u8::MAX as f32,
		)
	}
}

impl<RED, GREEN, BLUE> RgbLed<RED, GREEN, BLUE>
where
	RED: PwmPin<Duty = u32>,
	GREEN: PwmPin<Duty = u32>,
	BLUE: PwmPin<Duty = u32>,
{
	pub fn set_rgb(&mut self, r: f32, g: f32, b: f32) {
		<(Red, Green, Blue)>::set_rgb(self, r, g, b)
	}

	pub fn set_rgb_u8(&mut self, r: u8, g: u8, b: u8) {
		<(Red, Green, Blue)>::set_rgb_u8(self, r, g, b)
	}
}

macro_rules! accessor {
	( name: $name:ident, item: $item:ty, pin: $pin:ident ) => {
		pub struct $name;

		impl<E, RED, GREEN, BLUE> OutputAcessor<RED, GREEN, BLUE> for $name
		where
			$item: OutputPin<Error = E>,
		{
			type Error = E;

			fn set_state<S: Into<PinState>>(
				led: &mut RgbLed<RED, GREEN, BLUE>,
				state: S,
			) -> Result<(), Self::Error> {
				led.$pin.set_state(state.into())
			}

			fn set_low(
				led: &mut RgbLed<RED, GREEN, BLUE>,
			) -> Result<(), Self::Error> {
				led.$pin.set_low()
			}

			fn set_high(
				led: &mut RgbLed<RED, GREEN, BLUE>,
			) -> Result<(), Self::Error> {
				led.$pin.set_high()
			}
		}

		#[cfg(feature = "create")]
		impl<E, RED, GREEN, BLUE> StatefulOutputAcessor<RED, GREEN, BLUE>
			for $name
		where
			$item: StatefulOutputPin<Error = E>,
		{
			fn is_set_low(
				led: &RgbLed<RED, GREEN, BLUE>,
			) -> Result<bool, Self::Error> {
				led.$pin.is_set_low()
			}

			fn is_set_high(
				led: &RgbLed<RED, GREEN, BLUE>,
			) -> Result<bool, Self::Error> {
				led.$pin.is_set_high()
			}
		}

		#[cfg(feature = "create")]
		impl<E, RED, GREEN, BLUE> ToggleableOutputAcessor<RED, GREEN, BLUE>
			for $name
		where
			$item: ToggleableOutputPin<Error = E>,
		{
			type Error = E;

			fn toggle(led: &mut RgbLed<RED, GREEN, BLUE>) -> Result<(), E> {
				led.$pin.toggle()
			}
		}

		impl<D, RED, GREEN, BLUE> PwmAccessor<RED, GREEN, BLUE> for $name
		where
			$item: PwmPin<Duty = D>,
		{
			type Duty = D;

			fn disable(led: &mut RgbLed<RED, GREEN, BLUE>) {
				led.$pin.disable()
			}

			fn enable(led: &mut RgbLed<RED, GREEN, BLUE>) {
				led.$pin.enable()
			}

			fn get_duty(led: &RgbLed<RED, GREEN, BLUE>) -> Self::Duty {
				led.$pin.get_duty()
			}

			fn get_max_duty(led: &RgbLed<RED, GREEN, BLUE>) -> Self::Duty {
				led.$pin.get_max_duty()
			}

			fn set_duty(led: &mut RgbLed<RED, GREEN, BLUE>, duty: Self::Duty) {
				led.$pin.set_duty(duty)
			}
		}

		impl<RED, GREEN, BLUE> PwmAccessorExt<RED, GREEN, BLUE> for $name
		where
			$name: PwmAccessor<RED, GREEN, BLUE, Duty = u32>,
		{
			fn set_duty_frac(
				led: &mut RgbLed<RED, GREEN, BLUE>,
				fraction: f32,
			) {
				let fraction = fraction.min(1.);
				let duty =
					($name::get_max_duty(led) as f32 * fraction) as Self::Duty;
				$name::set_duty(led, duty)
			}
		}
	};
}

accessor!(name: Red, item: RED, pin: red);
accessor!(name: Green, item: GREEN, pin: green);
accessor!(name: Blue, item: BLUE, pin: blue);

macro_rules! accessor_combination {
        (
            $(
                name: $name:ident, item: $item:ty
            );+
        ) => {
            impl<E, RED, GREEN, BLUE> OutputAcessor<RED, GREEN, BLUE> for ( $( $name ),+ )
            where
                $(
                    $name: OutputAcessor<RED, GREEN, BLUE, Error = E>
                ),+
            {
                type Error = E;

                fn set_state<S: Into<PinState>>(
                    led: &mut RgbLed<RED, GREEN, BLUE>,
                    state: S,
                ) -> Result<(), Self::Error> {
                    let state = state.into();

                    $( $name::set_state(led, state)?; )+
                    Ok(())
                }

                fn set_low(led: &mut RgbLed<RED, GREEN, BLUE>) -> Result<(), Self::Error> {
                    $( $name::set_low(led)?; )+
                    Ok(())
                }

                fn set_high(led: &mut RgbLed<RED, GREEN, BLUE>) -> Result<(), Self::Error> {
                    $( $name::set_high(led)?; )+
                    Ok(())
                }
            }

			#[cfg(feature = "create")]
            impl<E, RED, GREEN, BLUE> StatefulOutputAcessor<RED, GREEN, BLUE> for ( $( $name ),+ )
            where
                $(
                    $name: StatefulOutputAcessor<RED, GREEN, BLUE, Error = E>
                ),+
            {
                fn is_set_low(led: &RgbLed<RED, GREEN, BLUE>) -> Result<bool, Self::Error> {
                    Ok( $( $name::is_set_low(led)? )&&+ )
                }

                fn is_set_high(led: &RgbLed<RED, GREEN, BLUE>) -> Result<bool, Self::Error> {
                    Ok( $( $name::is_set_high(led)? )&&+ )
                }
            }

			#[cfg(feature = "create")]
            impl<E, RED, GREEN, BLUE> ToggleableOutputAcessor<RED, GREEN, BLUE> for ( $( $name ),+ )
            where
                $(
                    $name: ToggleableOutputAcessor<RED, GREEN, BLUE, Error = E>
                ),+
            {
                type Error = E;

                fn toggle(led: &mut RgbLed<RED, GREEN, BLUE>) -> Result<(), Self::Error> {
                    $( $name::toggle(led)?; )+
                    Ok(())
                }
            }

            impl<D, RED, GREEN, BLUE> PwmAccessor<RED, GREEN, BLUE> for ( $( $name ),+ )
            where
                D: Clone,
                $(
                    $name: PwmAccessor<RED, GREEN, BLUE, Duty = D>
                ),+
            {
                type Duty = D;

                fn disable(led: &mut RgbLed<RED, GREEN, BLUE>) {
                    $( $name::disable(led); )+
                }

                fn enable(led: &mut RgbLed<RED, GREEN, BLUE>) {
                    $( $name::enable(led); )+
                }

                fn get_duty(_: &RgbLed<RED, GREEN, BLUE>) -> Self::Duty {
                    todo!()
                }

                fn get_max_duty(_: &RgbLed<RED, GREEN, BLUE>) -> Self::Duty {
                    todo!()
                }

                fn set_duty(led: &mut RgbLed<RED, GREEN, BLUE>, duty: Self::Duty) {
                    $( $name::set_duty(led, duty.clone()); )+
                }
            }

            impl<D, RED, GREEN, BLUE> PwmAccessorExt<RED, GREEN, BLUE> for ( $( $name ),+ )
            where
                D: Clone,
                $(
                    $name: PwmAccessorExt<RED, GREEN, BLUE, Duty = D>
                ),+
            {
                fn set_duty_frac(led: &mut RgbLed<RED, GREEN, BLUE>, fraction: f32) {
                    $( $name::set_duty_frac(led, fraction); )+
                }
            }
        };
    }

accessor_combination![name: Red, item: RED; name: Green, item: GREEN];
accessor_combination![name: Red, item: RED; name: Blue, item: BLUE];
accessor_combination![name: Green, item: GREEN; name: Red, item: RED];
accessor_combination![name: Green, item: GREEN; name: Blue, item: BLUE];
accessor_combination![name: Blue, item: BLUE; name: Red, item: RED];
accessor_combination![name: Blue, item: BLUE; name: Green, item: GREEN];

accessor_combination![name: Red, item: RED; name: Green, item: GREEN; name: Blue, item: BLUE];
accessor_combination![name: Red, item: RED; name: Blue, item: BLUE; name: Green, item: GREEN];
accessor_combination![name: Green, item: GREEN; name: Red, item: RED; name: Blue, item: BLUE];
accessor_combination![name: Green, item: GREEN; name: Blue, item: BLUE; name: Red, item: RED];
accessor_combination![name: Blue, item: BLUE; name: Red, item: RED; name: Green, item: GREEN];
accessor_combination![name: Blue, item: BLUE; name: Green, item: GREEN; name: Red, item: RED];

macro_rules! rgb_ext {
	(
            red: $red:ident, green: $green:ident, blue: $blue:ident
        ) => {
		impl<D, RED, GREEN, BLUE> PwmAccessorRgbExt<RED, GREEN, BLUE>
			for ($red, $green, $blue)
		where
			D: Clone,
			$red: PwmAccessorExt<RED, GREEN, BLUE, Duty = D>,
			$green: PwmAccessorExt<RED, GREEN, BLUE, Duty = D>,
			$blue: PwmAccessorExt<RED, GREEN, BLUE, Duty = D>,
		{
			fn set_rgb(
				led: &mut RgbLed<RED, GREEN, BLUE>,
				r: f32,
				g: f32,
				b: f32,
			) {
				$red::set_duty_frac(led, r);
				$green::set_duty_frac(led, g);
				$blue::set_duty_frac(led, b);
			}
		}

		impl<D, RED, GREEN, BLUE> PwmAccessorRgbExt<RED, GREEN, BLUE>
			for ($red, $blue, $green)
		where
			D: Clone,
			$red: PwmAccessorExt<RED, GREEN, BLUE, Duty = D>,
			$green: PwmAccessorExt<RED, GREEN, BLUE, Duty = D>,
			$blue: PwmAccessorExt<RED, GREEN, BLUE, Duty = D>,
		{
			fn set_rgb(
				led: &mut RgbLed<RED, GREEN, BLUE>,
				r: f32,
				g: f32,
				b: f32,
			) {
				$red::set_duty_frac(led, r);
				$green::set_duty_frac(led, g);
				$blue::set_duty_frac(led, b);
			}
		}

		impl<D, RED, GREEN, BLUE> PwmAccessorRgbExt<RED, GREEN, BLUE>
			for ($green, $blue, $red)
		where
			D: Clone,
			$red: PwmAccessorExt<RED, GREEN, BLUE, Duty = D>,
			$green: PwmAccessorExt<RED, GREEN, BLUE, Duty = D>,
			$blue: PwmAccessorExt<RED, GREEN, BLUE, Duty = D>,
		{
			fn set_rgb(
				led: &mut RgbLed<RED, GREEN, BLUE>,
				r: f32,
				g: f32,
				b: f32,
			) {
				$red::set_duty_frac(led, r);
				$green::set_duty_frac(led, g);
				$blue::set_duty_frac(led, b);
			}
		}

		impl<D, RED, GREEN, BLUE> PwmAccessorRgbExt<RED, GREEN, BLUE>
			for ($green, $red, $blue)
		where
			D: Clone,
			$red: PwmAccessorExt<RED, GREEN, BLUE, Duty = D>,
			$green: PwmAccessorExt<RED, GREEN, BLUE, Duty = D>,
			$blue: PwmAccessorExt<RED, GREEN, BLUE, Duty = D>,
		{
			fn set_rgb(
				led: &mut RgbLed<RED, GREEN, BLUE>,
				r: f32,
				g: f32,
				b: f32,
			) {
				$red::set_duty_frac(led, r);
				$green::set_duty_frac(led, g);
				$blue::set_duty_frac(led, b);
			}
		}

		impl<D, RED, GREEN, BLUE> PwmAccessorRgbExt<RED, GREEN, BLUE>
			for ($blue, $red, $green)
		where
			D: Clone,
			$red: PwmAccessorExt<RED, GREEN, BLUE, Duty = D>,
			$green: PwmAccessorExt<RED, GREEN, BLUE, Duty = D>,
			$blue: PwmAccessorExt<RED, GREEN, BLUE, Duty = D>,
		{
			fn set_rgb(
				led: &mut RgbLed<RED, GREEN, BLUE>,
				r: f32,
				g: f32,
				b: f32,
			) {
				$red::set_duty_frac(led, r);
				$green::set_duty_frac(led, g);
				$blue::set_duty_frac(led, b);
			}
		}

		impl<D, RED, GREEN, BLUE> PwmAccessorRgbExt<RED, GREEN, BLUE>
			for ($blue, $green, $red)
		where
			D: Clone,
			$red: PwmAccessorExt<RED, GREEN, BLUE, Duty = D>,
			$green: PwmAccessorExt<RED, GREEN, BLUE, Duty = D>,
			$blue: PwmAccessorExt<RED, GREEN, BLUE, Duty = D>,
		{
			fn set_rgb(
				led: &mut RgbLed<RED, GREEN, BLUE>,
				r: f32,
				g: f32,
				b: f32,
			) {
				$red::set_duty_frac(led, r);
				$green::set_duty_frac(led, g);
				$blue::set_duty_frac(led, b);
			}
		}
	};
}

rgb_ext!(red: Red, green: Green, blue: Blue);
