trait Keeper{

	/// Generate initial internal state data from given World/Surface
	/// Info.
	
	fn generateInitState() -> Self;
	
	// It looks like generators/coroutines aren't supported on
	// mainline Rust, so we can't have generateState() function as
	// one. A pity. But maybe refactor eventually?

	/// Regenerate internal state data, but this time taking more
	/// advanced simulation factors into account.
	
	fn reGenState() -> Self;
	
	/// Generate a Moulding object from internal state data, and
	/// return a static reference to it.

	// Technically, we aren't *generating* a new moulding so much as
	// we are giving access to a reference to a consistently stored
	// moulding maintained by the Keeper. This means that this method
	// should only need to be called once, as once a reference to the
	// moulding is acquired, we can just keep on reapplying the same
	// moulding to the WorldShaper at regular intervals, and the
	// simulation should proceed accordingly. Nice!

	fn generateMoulding() -> &'static Moulding;
}
