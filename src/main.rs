#![allow(dead_code, non_snake_case)]

mod AltaTerra;
mod Precipitation;

//mod clap;
//mod tinybmp;

use crate::AltaTerra::{
	TerraInfo,
};

use crate::Precipitation::{
	RainKeeper,
};

// use embedded_graphics::prelude::*;
//use tinybmp::{RawBmp, Bpp, Header, RawPixel, RowOrder};

fn main(){

	// First, instantiate a TerraInfo object. Fill out some basic
	// information about map zone, and then generate the processable
	// worldInfo and surfaceInfo.
	
	let terraInfo: TerraInfo = TerraInfo::createTerra("Terra1".to_string() , 100, 50, 8, 15);
	let rainKeeper: RainKeeper = RainKeeper::init(&terraInfo);
	
	println!("Generated Terra!");

	
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
