//! C wrapper for [Rant](`rant`).

#![doc(html_root_url = "https://docs.rs/rantsy/0.0.1")]
#![warn(clippy::pedantic)]
#![warn(missing_docs)]

use tap::Pipe;

#[cfg(doctest)]
pub mod readme {
	doc_comment::doctest!("../README.md");
}

trait Helpers: Sized {
	fn box_to_c(self) -> &'static mut Self {
		self.pipe(Box::new).pipe(Box::leak)
	}

	unsafe fn drop_from_c(&'static mut self) {
		Box::from_raw(self as *mut Self).pipe(drop)
	}
}
impl<T: Sized> Helpers for T {}

pub mod rant;
pub mod types;
