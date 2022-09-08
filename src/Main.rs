#![allow(dead_code)]

mod AltaTerra;
mod AlterTerra;

mod clap;
mod tinybmp;


use AltaTerra::MAX_COORD as MAX_COORD;

use AlterTerra::{
	WorldShaper,
	Moulding
};

// use embedded_graphics::prelude::*;
use tinybmp::{RawBmp, Bpp, Header, RawPixel, RowOrder};

fn Main(){

	// First, instantiate a TerraInfo object. Fill out some basic
	// information about map zone, and then generate the processable
	// worldInfo and surfaceInfo.
	
	let terraInfo: TerraInfo = TerraInfo{
		name: "TestTerra",
		xCoords: AltaTerra::MAX_COORD,
		yCoords: AltaTerra::MAX_COORD,
		zCoords: AltaTerra::MAX_COORD,
		cellMeasure: 5 };

	let tmp = terraInfo.createTerra();
	let worldInfo: WorldInfo = tmp.0;
	let surfaceInfo: SurfaceInfo = tmp.1;


	let worldShaper: WorldShaper = WorldShaper{
		terraInfo: terraInfo,
		worldInfo: worldInfo,
		surfaceInfo: surfaceInfo
	};

	// rkc is a tuple containing map extents for this run.
	let rkc = terraInfo.getExtents();

	
	// Next, instantiate a RainKeeper object with information from
	// terraInfo, and pass into it the worldInfo and surfaceInfo.

	let RainKeeper: rainKeeper = RainKeeper{
		mWindow: &[&[RainInfo; 0]; 0];
	};

	rainKeeper.generateInitState(rkc.0, rkc.1, rkc.2);
	

	
	// ..

	// Run the simulation.
	startSimulation(/* params */);
}
