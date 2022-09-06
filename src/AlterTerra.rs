#![allow(dead_code)]

mod AltaTerra;

struct MouldInfo{
	//! Information that is used to modify the data inside of a
	//! CellInfo.

	/// Downard Force of the moulding effect.
	downMag: u16,

	/// Horizontal orientation vector of the moulding effect.
	orient: f32,

	/// Soil Displacement per unit of force.
	dispForce: f32,
}

struct Moulding{
	/// coordinates of the moulding transformation.
	xOrigin: u16,
	yOrigin: u16,

	// Note: Mouldings are always applied to the surface of the world,
	// so there is no zOrigin.
	
	/// A 2D vector of changes to be applied to the
	/// surface of a world.
	mCoords: [mut [mut u16, MAX_COORD], MAX_COORD],
	mut mWindow: &[&[u16]];
}

struct WorldShaper{
	/// Information about the space the world inhabits.
	AltaTerra::TerraInfo: terraInfo,
	
	/// Information about the nature of the world itself.
	AltaTerra::WorldInfo: worldInfo,

	/// Information about the surface of world ground.
	AltaTerra::SurfaceInfo: surfaceInfo,
}

impl WorldShaper{
	fn shapeWorld(&self, data: &Vec<Vec<Moulding>>) {
		
	}
}
