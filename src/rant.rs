use crate::Helpers;
use rant::{runtime::RuntimeResult, Rant, RantProgram, RantValue};

/// Constructor.
#[no_mangle]
pub extern "C" fn rantsy_rant_new() -> &'static mut Rant {
	Rant::new().box_to_c()
}

/// See [`Rant::clear_data_sourced`].
#[no_mangle]
pub extern "C" fn rantsy_rant_clear_data_sources(this: &mut Rant) {
	this.clear_data_sources()
}

/// See [`Rant::run`].
#[no_mangle]
pub extern "C" fn rantsy_rant_run(
	this: &mut Rant,
	program: &RantProgram,
) -> &'static mut RuntimeResult<RantValue> {
	this.run(&program).box_to_c()
}

/// # Safety
///
/// Must only be called with a reference created using [`rantsy_rant_new`].
/// Invalidates it.
#[no_mangle]
pub unsafe extern "C" fn rantsy_rant_drop(this: &'static mut Rant) {
	this.drop_from_c()
}
