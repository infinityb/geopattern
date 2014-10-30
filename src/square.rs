use std::rand::{Rng, Rand};
use std::default::Default;

use render::Render;
use seed::{PatternSeed, FromSeed};
use surface::Surface;
use utils;

const SQUARES_ACROSS: uint = 6;
const SQUARES_AREA: uint = SQUARES_ACROSS * SQUARES_ACROSS;

pub struct SquarePattern {
	square_size: uint,
	square_data: [u8, ..SQUARES_AREA]
}

impl Default for SquarePattern {
	fn default() -> SquarePattern {
		SquarePattern {
			square_size: 10,
			square_data: [0, ..SQUARES_AREA],
		}
	}
}

impl FromSeed for SquarePattern {
	fn from_seed<S: PatternSeed>(seed: &S) -> SquarePattern {
		unimplemented!();
	}
}

impl Rand for SquarePattern {
	fn rand<R: Rng>(rng: &mut R) -> SquarePattern {
		let square_size = 10 + 4 * rng.gen_range(0u, 16);

		let mut square_data = [0, ..SQUARES_AREA];
		for i in range(0u, SQUARES_AREA) {
			square_data[i] = rng.gen_range(0_u8, 16);
		}

		SquarePattern {
			square_size: square_size,
			square_data: square_data
		}
	}
}

impl Render for SquarePattern {
	fn size_2d(&self) -> (uint, uint) {
		let side_len = self.square_size * SQUARES_ACROSS;
		(side_len, side_len)
	}

	fn render(&self, surf: &mut Surface) {
		for y in range(0u, SQUARES_ACROSS) {
			let y_off = y * self.square_size;
			for x in range(0u, SQUARES_ACROSS) {
				let x_off = x * self.square_size;
				let val = self.square_data[y * SQUARES_ACROSS + x];
				let opacity: f64 = utils::opacity(val as f64 / 15.0);
				let fill: (f64, f64, f64) = utils::fill_color(val as uint);

				unimplemented!(); // paint
			}
		}
	}
}