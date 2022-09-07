#![allow(dead_code)]

mod AltaTerra;
mod AlterTerra;

mod clap;
mod tinybmp;


use AltaTerra::MAX_COORD as MAX_COORD;

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

	// Next, instantiate a RainKeeper object with information from
	// terraInfo, and pass into it the worldInfo and surfaceInfo.

	let RainKeeper: rainKeeper = RainKeeper{/* attributes */}
	
	// ..

	// Run the simulation.
	startSimulation(/* params */);
}
