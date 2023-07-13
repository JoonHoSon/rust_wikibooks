use std::num::Wrapping;

pub fn rand(seed: &mut u32) -> u32 {
	let (a, c) = (322823242u32, 57383123u32);

	*seed = (Wrapping(*seed) * Wrapping(a) + Wrapping(c)).0;

	return *seed;
}