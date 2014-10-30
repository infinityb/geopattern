use surface::Surface;

pub trait Render {
	// return natural size
	fn size_2d(&self) -> (uint, uint);

	// render to a surface
	fn render(&self, surf: &mut Surface);
}
