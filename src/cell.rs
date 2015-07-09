//! This module contains definition of table/row cells stuff

use std::io::{Write, Error};
use std::string::ToString;
use unicode_width::UnicodeWidthStr;

/// Represent a table cell containing a string.
///
/// Once created, a cell's content cannot be modified.
/// The cell would have to be replaced by another one
#[derive(Clone, Debug)]
pub struct Cell {
	content: Vec<String>,
	width: usize
}

impl Cell {
	/// Create a new `Cell` initialized with content from `string`
	pub fn new(string: &String) -> Cell {
		let content: Vec<String> = string.lines_any().map(|ref x| x.to_string()).collect();
		let mut width = 0;
		for cont in &content {
			let l = UnicodeWidthStr::width(&cont[..]);
			if l > width {
				width = l;
			}
		}
		return Cell {
			content: content,
			width: width
		};
	}
	
	/// Return the height of the cell
	pub fn get_height(&self) -> usize {
		return self.content.len();
	}
	
	/// Return the width of the cell
	pub fn get_width(&self) -> usize {
		return self.width;
	}
	
	/// Return a copy of the full string contained in the cell
	pub fn get_content(&self) -> String {
		return self.content.iter().fold("".to_string(), (|acc, ref item| format!("{}\n{}", acc, item)));
	}
	
	/// Print a partial cell to `out`. Since the cell may be multi-lined,
	/// `idx` is the line index to print. `col_width` is the column width used to
	/// fill the cells with blanks so it fits in the table.
	/// If `ìdx` is higher than this cell's height, it will print empty content
	pub fn print<T: Write>(&self, out: &mut T, idx: usize, col_width: usize) -> Result<(), Error> {
		try!(out.write_all(b" "));
		let mut len = 0;
		if let Some(content) = self.content.get(idx) {
			try!(out.write_all(content.as_bytes()));
			len = UnicodeWidthStr::width(&content[..]);
		}
		try!(out.write_all(&vec![' ' as u8; col_width - len + 1]));
		return Ok(());
	}
}

impl <'a, T: ToString> From<&'a T> for Cell {
	fn from(f: &T) -> Cell {
		return Cell::new(&f.to_string());
	}
}

impl ToString for Cell {
	fn to_string(&self) -> String {
		return self.get_content();
	}
}

impl Default for Cell {
	/// Return a cell initialized with a single empty `String`
	fn default() -> Cell {
		return Cell {
			content: vec!["".to_string(); 1],
			width: 0
		};
	}
}

#[cfg(test)]
mod tests {
	use cell::Cell;
	use utils::StringWriter;

	#[test]
	fn ascii() {
		let ascii_cell = Cell::new(&String::from("hello"));
		assert_eq!(ascii_cell.get_width(), 5);

		let mut out = StringWriter::new();
		let _ = ascii_cell.print(&mut out, 0, 10);
		assert_eq!(out.as_string(), " hello      ");
	}

	#[test]
	fn unicode() {
		let unicode_cell = Cell::new(&String::from("привет"));
		assert_eq!(unicode_cell.get_width(), 6);

		let mut out = StringWriter::new();
		let _ = unicode_cell.print(&mut out, 0, 10);
		assert_eq!(out.as_string(), " привет     ");
	}
}

