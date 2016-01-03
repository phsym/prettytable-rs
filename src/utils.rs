//! Internal only utilities
use std::io::{Error, ErrorKind, Write};
use std::str;

use unicode_width::UnicodeWidthStr;

use super::format::Align;

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

/// Align/fill a string and print it to `out`
pub fn print_align<T: Write+?Sized>(out: &mut T, align: Align, text: &str, fill: char, size: usize) -> Result<(), Error> {
	let text_len = UnicodeWidthStr::width(text);
	let mut nfill = if text_len < size { size - text_len } else { 0 };
	match align {
		Align::LEFT => {},
		Align:: RIGHT => {try!(out.write(&vec![fill as u8; nfill])); nfill = 0;},
		Align:: CENTER => {try!(out.write(&vec![fill as u8; nfill/2])); nfill -= nfill/2;}
	}
	try!(out.write(text.as_bytes()));
	try!(out.write(&vec![fill as u8; nfill]));
	return Ok(());
}

#[cfg(test)]
mod tests {
	use super::*;
	use format::Align;
	use std::io::Write;

	#[test]
	fn string_writer() {
		let mut out = StringWriter::new();
		out.write("foo".as_bytes()).unwrap();
		out.write(" ".as_bytes()).unwrap();
		out.write("".as_bytes()).unwrap();
		out.write("bar".as_bytes()).unwrap();
		assert_eq!(out.as_string(), "foo bar");
	}

	#[test]
	fn fill_align() {
		let mut out = StringWriter::new();
		print_align(&mut out, Align::RIGHT, "foo", '*', 10).unwrap();
		assert_eq!(out.as_string(), "*******foo");

		let mut out = StringWriter::new();
		print_align(&mut out, Align::LEFT, "foo", '*', 10).unwrap();
		assert_eq!(out.as_string(), "foo*******");

		let mut out = StringWriter::new();
		print_align(&mut out, Align::CENTER, "foo", '*', 10).unwrap();
		assert_eq!(out.as_string(), "***foo****");

		let mut out = StringWriter::new();
		print_align(&mut out, Align::CENTER, "foo", '*', 1).unwrap();
		assert_eq!(out.as_string(), "foo");
	}
}
