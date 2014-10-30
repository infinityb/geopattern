pub trait PatternSeed {
	fn gen_range(&self, low: uint, high: uint) -> uint;
}

impl PatternSeed for Vec<u8> {
	fn gen_range(&self, low: uint, high: uint) -> uint {
		unimplemented!();
	}
}

pub trait FromSeed {
	fn from_seed<S: PatternSeed>(seed: &S) -> Self;
}