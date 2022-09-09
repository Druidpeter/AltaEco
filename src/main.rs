#![allow(dead_code, non_snake_case)]

mod AltaTerra;

//mod clap;
//mod tinybmp;

use AltaTerra::{
	TerraInfo,
};

/*
use AlterTerra::{
	WorldShaper,
	Moulding
};
*/

// use embedded_graphics::prelude::*;
//use tinybmp::{RawBmp, Bpp, Header, RawPixel, RowOrder};

fn main(){

	// First, instantiate a TerraInfo object. Fill out some basic
	// information about map zone, and then generate the processable
	// worldInfo and surfaceInfo.
	
	let terraInfo: TerraInfo = TerraInfo::createTerra("Terra1".to_string() , 100, 50, 8, 15);

	println!("Generated Terra!");

	// let worldShaper: WorldShaper = WorldShaper{
	// 	terraInfo: terraInfo,
	// 	worldInfo: worldInfo,
	// 	surfaceInfo: surfaceInfo
	// };

	// // rkc is a tuple containing map extents for this run.
	// let rkc = terraInfo.getExtents();

	
	// // Next, instantiate a RainKeeper object with information from
	// // terraInfo, and pass into it the worldInfo and surfaceInfo.

	// let RainKeeper: rainKeeper = RainKeeper{
	// 	mWindow: &[&[RainInfo; 0]; 0];
	// };

	// rainKeeper.generateInitState(rkc.0, rkc.1, rkc.2);
	

	
	// // ..

	// // Run the simulation.
	// startSimulation(/* params */);
}
