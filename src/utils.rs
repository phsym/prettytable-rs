//! Internal only utilities
use std::io::{Error, ErrorKind, Write};
use std::str;

#[cfg(not(windows))]
pub static NEWLINE: &'static [u8] = b"\n";
#[cfg(windows)]
pub static NEWLINE: &'static [u8] = b"\r\n";

/// Internal utility for writing data into a string
pub struct StringWriter {
	string: String
}

impl StringWriter {
	/// Create a new `StringWriter`
	pub fn new() -> StringWriter {
		return StringWriter{string: String::new()};
	}
	
	/// Return a reference to the internally written `String`
	pub fn as_string(&self) -> &str {
		return &self.string;
	}
}

impl Write for StringWriter {
	fn write(&mut self, data: &[u8]) -> Result<usize, Error> {
		let string = match str::from_utf8(data) {
			Ok(s) => s,
			Err(e) => return Err(Error::new(ErrorKind::Other, format!("Cannot decode utf8 string : {}", e)))
		};
		self.string.push_str(string);
		return Ok(data.len());
	}
	
	fn flush(&mut self) -> Result<(), Error> {
		// Nothing to do here
		return Ok(());
	}
}