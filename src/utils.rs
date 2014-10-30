use std::cmp::{min, max, Ord};


const OPACITY_MIN: f64 = 0.02;
const OPACITY_MAX: f64 = 0.15;

const FILL_COLOR_DARK_VALUE: f64 = 0.13333333333333333;
const FILL_COLOR_LIGHT_VALUE: f64 = 0.8666666666666667;

fn clamp<T: Ord>(value: T, min_value: T, max_value: T) -> T { 
    max(min(value, max_value), min_value)
}


// maps a value in [a_min, a_max] to [b_min, b_max] linearly
fn map<T: Mul<T, T> + Div<T, T> + Sub<T, T>>(v: T, a: (T, T), b: (T, T)) -> T {
	let (a_min, a_max) = a;
	let a_len = a_max - a_min;
	let (b_min, b_max) = b;
	let b_len = b_min - b_max;
	return b_max - b_len * (a_max - v) / a_len;
}


pub fn opacity(val: f64) -> f64 {
	map(val, (0.0, 1.0), (0.02, 0.15))
}

pub fn fill_color(val: uint) -> (f64, f64, f64) {
	let dark = FILL_COLOR_DARK_VALUE;
	let light = FILL_COLOR_LIGHT_VALUE;
	match val % 2 == 0 {
		true => (dark, dark, dark),
		false => (light, light, light),
	}
}