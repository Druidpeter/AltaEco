#![allow(dead_code, non_snake_case)]

use crate::AltaTerra::{
	TerraInfo,
	Moulding,
};

#[derive(Clone)]
struct RainInfo{
	dewPoint: f32,
	humidity: f32,
	precipitation: f32,
}

impl RainInfo{
	fn init() -> RainInfo {
		RainInfo{
			dewPoint: 0.0,
			humidity: 0.0,
			precipitation: 0.0,
		}
	}
}

/// Keeps track of precipitation levels over the world.

pub struct RainKeeper{
	wCoords: Vec<RainInfo>,
	wBuffer: Vec<RainInfo>,
}

impl RainKeeper{
	pub fn init(wData: &TerraInfo) -> RainKeeper {
		let bds = wData.getBounds1D();
		
		RainKeeper{
			wCoords: vec![RainInfo::init(); bds],
			wBuffer: vec![RainInfo::init(); bds],
		}
	}
}

trait Keeper{	
	// It looks like generators/coroutines aren't supported on
	// mainline Rust, so we can't have generateState() function as
	// one. A pity. But maybe refactor eventually.
	
	fn reGenState(&self, wData: &TerraInfo);
	
	// Generate a Moulding object from internal state data, and
	// return a static reference to it.

	// Technically, we aren't *generating* a new moulding so much as
	// we are giving access to a reference to a consistently stored
	// moulding maintained by the Keeper. This means that this method
	// should only need to be called once, as once a reference to the
	// moulding is acquired, we can just keep on reapplying the same
	// moulding to the WorldShaper at regular intervals, and the
	// simulation should proceed accordingly. Nice!

	// fn generateMoulding(wData: &TerraInfo) -> Moulding;
}

impl Keeper for RainKeeper{
	fn reGenState(&self, wData: &TerraInfo){
		//! Normally, this is where a bunch of the simulation logic
		//! for precipitation would run. Since we're just starting
		//! out, however, we don't want to get too crazy and/or in the
		//! weeds with writing our logic. Something nice and simple
		//! will allow us to debug everything else more cleanly, and
		//! we can add the advanced stuff in after.
	}

	// fn generateMoulding(wData: &TerraInfo) -> Moulding {
		
	// }
}
