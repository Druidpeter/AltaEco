
#![allow(dead_code)]

pub const MAX_COORD: u16 = 16000;

struct TerraInfo{
	name: String,

	// The xy dimensions, equal to the pixel size dimensions of the
	// input heightmap.

	// The z dimension is input directly, either via command-line
	// option, or via GUI.
	
	xCoords: u16,
	yCoords: u16,
	zCoords: u16,
	
	// How wide/long/tall each cell is.
	// No point in going larger than 256 meter cells.
	cellMeasure: u8,
}

impl TerraInfo{
	fn createTerra(&self) -> (WorldInfo, SurfaceInfo) { 
		//! Return WorldInfo and SurfaceInfo objects generated from
		//! own data.
	}
}

struct SurfaceInfo{
	// Likely this needs to be adjusted, but close enough to get the
	// idea for now.

	/// Essentially a 2D-Array that contains indexes to the cells
	/// which represent the surface-air boundary between the ground
	/// and sky.

	// As cells lose soil due to erosion, the value of an xy cell here
	// is updated to lower z-values. If a specific xy cell has a z
	// value of '0', then that means that the ground at that location
	// has been eroded to sea-level, and no more erosion processing is
	// possible.
	
	sCoords: [mut [mut u16, MAX_COORD], MAX_COORD];

	// Note: Try and convert the sCoords vector into an immutable
	// array with mutable cells.
}

/// Standard LS Factor is defined as a 9% slope from top to bottom on
/// a 22.13x22.13 plot of soil. The actual LS Factor for a given cell
/// is determined by multiplying the plotSlope and cell length
/// together, and then taking the ratio of the result over the
/// StandardLS Factor.
let StandardLSF: = 0.09 * 22.13;

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

struct WorldInfo{
	/// A 3D-Array that contains CellInfos.

	wCoords: [mut [mut [mut CellInfo, MAX_COORD], MAX_COORD], MAX_COORD];
}

impl WorldInfo{
	fn genSurfaceNoise(&self){
		//! Slightly randomize soil Erodibility and supportFactor values on a per
		//! cell basis.
	}
}
