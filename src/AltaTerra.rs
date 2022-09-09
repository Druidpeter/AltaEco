#![allow(dead_code, non_snake_case)]

/// Standard LS Factor is defined as a 9% slope from top to bottom on
/// a 22.13x22.13 plot of soil. The actual LS Factor for a given cell
/// is determined by multiplying the plotSlope and cell length
/// together, and then taking the ratio of the result over the
/// StandardLS Factor.
const STANDARD_LSF: f32 = 0.09 * 22.13;

#[derive(Clone)]
struct CellInfo{
	soilErodibility: f32,

	/// If any soil has been removed from this cell, keep track of it
	/// here.
	soilEroded: f64,

	/// If any soil has been added to this cell, keep track of it
	/// here.
	soilDeposited: f64,
	
	/// Amount of soil in kg/m^3.
	soilDensity: f32,

	/// Up/Down slope as percent drop in height between uppermost
	/// point in cell and lowermost point in cell.
	plotSlope: f32,

	/// Horizontal orientation of the vector determined by the line
	/// passing through the uppermost point of the plot and the
	/// lowermost point of the plot, with the head of the vector on
	/// the lowermost point.
	slopeDirection: f32,

	/// Direct hinting of soil-loss through a specific value. These
	/// values would normally correspond to tillage practices in the
	/// real world, but for our use cases, they are provided by a
	/// separate overlay image of the same dimensions as that as the
	/// given height-map.
	
	supportFactor: f32,
}

impl CellInfo{
	fn init() -> CellInfo{
		CellInfo{
			soilErodibility: 0.0,
			soilEroded: 0.0,
			soilDeposited: 0.0,
			soilDensity: 0.0,
			plotSlope: 0.0,
			slopeDirection: 0.0,
			supportFactor: 0.0
		}
	}
}

pub struct TerraInfo{
	name: String,

	// The xy dimensions, equal to the pixel size dimensions of the
	// input heightmap.

	// The z dimension is input directly, either via command-line
	// option, or via GUI.
	
	xCoords: usize,
	yCoords: usize,
	zCoords: usize,
	
	// How wide/long/tall each cell is.
	// No point in going larger than 256 meter cells.
	cellMeasure: u8,

	/// A 3D-Array that contains CellInfos.
	wCoords: Vec<CellInfo>,
	
	/// Essentially a 2D-Array that contains indexes to the cells
	/// which represent the surface-air boundary between the ground
	/// and sky.

	// As cells lose soil due to erosion, the value of an xy cell here
	// is updated to lower z-values. If a specific xy cell has a z
	// value of '0', then that means that the ground at that location
	// has been eroded to sea-level, and no more erosion processing is
	// possible.
	
	sCoords: Vec<u16>,
}

impl TerraInfo{
	pub fn createTerra(nm: String, xc: usize, yc: usize, zc: usize, measure: u8) -> TerraInfo {
		//! Return WorldInfo and SurfaceInfo objects generated from
		//! own data.

		TerraInfo{
			name: nm,
			xCoords: xc,
			yCoords: yc,
			zCoords: zc,
			cellMeasure: measure,
			wCoords: vec![CellInfo::init(); xc*yc*zc],
			sCoords: vec![0; xc*yc],
		}
	}

	fn genSurfaceNoise(&self){
		//! Slightly randomize soil Erodibility and supportFactor values on a per
		//! cell basis.
	}

	fn applyMoulding(&self, data: Moulding){
		//! Apply contents of the moulding to the terrain.
	}

	pub fn getBounds1D(&self) -> usize{
		self.xCoords*self.yCoords*self.zCoords
	}
}

/// Information that is used to modify the data inside of a
/// CellInfo.

#[derive(Clone)]
struct MouldInfo{
	/// Downard Force of the moulding effect.
	downMag: u16,

	/// Horizontal orientation vector of the moulding effect.
	orient: f32,

	/// Soil Displacement per unit of force.
	dispForce: f32,
}

impl MouldInfo{
	fn init() -> MouldInfo{
		MouldInfo{
			downMag: 0,
			orient: 0.0,
			dispForce: 0.0,
		}
	}
}

pub struct Moulding{
	/// coordinates of the moulding transformation.
	xOrigin: usize,
	yOrigin: usize,
	zOrigin: usize,

	xXtent: usize,
	yXtent: usize,
	zXtent: usize,
	

	// Note: Mouldings are always applied to the surface of the world,
	// so there is no zOrigin.
	
	/// A 2D vector of changes to be applied to the
	/// surface of a world.
	mCoords: Vec<MouldInfo>,
}

impl Moulding{
	pub fn createMoulding(xo: usize, yo: usize, zo: usize, xt: usize, yt: usize, zt: usize) -> Moulding {
		Moulding {
			xOrigin: xo,
			yOrigin: yo,
			zOrigin: zo,

			xXtent: xt,
			yXtent: yt,
			zXtent: zt,

			mCoords: vec![MouldInfo::init(); (xt-xo)*(yt-yo)],
		}
	}
}
