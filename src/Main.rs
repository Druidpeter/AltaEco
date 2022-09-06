#![allow(dead_code)]

mod AltaTerra;
mod AlterTerra;

mod clap;
mod tinybmp;


use AltaTerra::MAX_COORD as MAX_COORD;

// use embedded_graphics::prelude::*;
use tinybmp::{RawBmp, Bpp, Header, RawPixel, RowOrder};

fn Main(){
	let terraInfo: TerraInfo = TerraInfo{
		name: "TestTerra",
		xCoords: AltaTerra::MAX_COORD,
		yCoords: AltaTerra::MAX_COORD,
		zCoords: AltaTerra::MAX_COORD,
		cellMeasure: 5 };

	let tmp = terraInfo.createTerra();
	let worldInfo: WorldInfo = tmp.0;
	let surfaceInfo: SurfaceInfo = tmp.1;
}
