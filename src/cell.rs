use std::io::{Write, Error};

/// Represent a cell in a table
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
		};
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
	
	/// Print a partial cell to `out`. Since the cell may be multi-lined,
	/// `idx` is the line index to print. `col_width` is the column width used to
	/// fill the cells with blanks so it fits in the table.
	/// If `ìdx` is higher than this cell's height, it will print empty content
	pub fn print<T: Write>(&self, out: &mut T, idx: usize, col_width: usize) -> Result<(), Error> {
		try!(out.write_all(b" "));
		let mut len = 0;
		if idx < self.get_height() {
			try!(out.write_all(self.content[idx].as_bytes()));
			len = self.content[idx].len();
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