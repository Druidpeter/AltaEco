mod AlterTerra;
mod Traits;

use Traits{
	Keeper
};

struct RainInfo{
	dewPoint: f32,
	humidity: f32,
	precipitation: f32,
}

struct RainKeeper{
	//! Keeps track of precipitation levels over the world.

	wCoords: [mut [mut [mut RainInfo, MAX_COORD], MAX_COORD], MAX_COORD];
	mut wWindow: &[&[RainInfo]];
}

impl Keeper for RainKeeper{
	fn generateInitState() -> Self{
		//
	}
	
	fn reGenState() -> Self{
		//! Normally, this is where a bunch of the simulation logic
		//! for precipitation would run. Since we're just starting
		//! out, however, we don't want to get too crazy and/or in the
		//! weeds with writing our logic. Something nice and simple
		//! will allow us to debug everything else more cleanly, and
		//! we can add the advanced stuff in after.

		// Additional Note: It might be nice to set up a registration
		// system with callbacks for specific values that are outside
		// of the domain of the keeper to calculate.

		// So, for example, the RainKeeper will require the air
		// temperature to properly calculate dew point and total
		// moisture content inside of a cell. But that information
		// isn't handled by the RainKeeper, but instead the
		// HeatKeeper.

		// It would make sense to permit the RainKeeper to
		// maintain/guess a basic default value, but then also provide
		// a method that lets an outside source provide information on
		// that value when needed (i.e. each simulation step) by
		// essentially giving the object a callback method for it to
		// call when that/those value(s) is/are required.

		// This way, when performing the simulation step, each Keeper
		// is capable of operating completely in isolation to
		// itself. But, the main program thread *can* essentially hook
		// up multiple different Keepers to work together at its
		// discretion, which provides a great deal of flexibility in
		// addition to encapsulation.

		// But all of this is a set of considerations for much
		// later. Right now, just focus on getting basic functionality
		// up and running.
	}

	fn generateMoulding() -> &'static Moulding{
		
	}
}
