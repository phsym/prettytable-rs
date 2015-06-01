//! This module contains definition of table/row cells stuff
use std::io::{Write, Error};

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
		let content: Vec<String> = string.lines_any().map(|x| x.to_string()).collect();
		let mut width = 0;
		for cont in &content {
			let l = cont.len();
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
			len = content.len();
		}
		try!(out.write_all(b" "));
		for _ in 0..(col_width - len) {
			try!(out.write_all(b" "));
		}
		return Ok(());
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