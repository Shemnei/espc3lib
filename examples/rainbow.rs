/// Needs the feature `create` enabled
use espc3lib::prelude::*;

fn red(angle: u32) -> f32 {
	let angle = angle as f32;

	if angle < 60. {
		1.
	} else if angle < 120. {
		((120. - angle) * 4.25 - 0.01) / 255.
	} else if angle < 240. {
		0.
	} else if angle < 300. {
		((angle - 240.) * 4.25 - 0.01) / 255.
	} else {
		1.
	}
}

fn green(angle: u32) -> f32 {
	let angle = angle as f32;

	if angle < 60. {
		(angle * 4.25 - 0.01) / 255.
	} else if angle < 180. {
		1.
	} else if angle < 240. {
		((240. - angle) * 4.25 - 0.01) / 255.
	} else {
		0.
	}
}

fn blue(angle: u32) -> f32 {
	let angle = angle as f32;

	if angle < 120. {
		0.
	} else if angle < 180. {
		((angle - 120.) * 4.25 - 0.01) / 255.
	} else if angle < 300. {
		1.
	} else {
		((360. - angle) * 4.25 - 0.01) / 255.
	}
}

fn main() -> ! {
	const FPS: f32 = 60.;

	esp_idf_sys::link_patches();

	let mut led = unsafe { RgbLed::pwm() }.unwrap();

	loop {
		led.set_rgb(0., 0., 0.);
		for cycle in 0..360 {
			led.set_rgb(red(cycle), green(cycle), blue(cycle));
			std::thread::sleep(std::time::Duration::from_millis(
				(1000. / FPS) as u64,
			));
		}
	}
}
