//! This module contains definition of table rows stuff
use std::io::{Write, Error};

use super::utils::LINEFEED;
use super::cell::Cell;

/// Represent a table row made of cells
#[derive(Clone, Debug)]
pub struct Row {
	cells: Vec<Cell>
}

impl Row {
	/// Create a new `Row` backed with `cells` vector
	pub fn new(cells: Vec<Cell>) -> Row {
		return Row {
			cells: cells
		};
	}
	
	/// Create an row of length `size`, with empty strings stored
	pub fn empty(size: usize) -> Row {
		return Self::new(vec![Cell::default(); size]);
	}
	
	/// Get the number of cells in this row
	pub fn len(&self) -> usize {
		return self.cells.len();
	}
	
	/// Get the height of this row
	pub fn get_height(&self) -> usize {
		let mut height = 0;
		for cell in &self.cells {
			let h = cell.get_height();
			if h > height {
				height = h;
			}
		}
		return height;
	}
	
	/// Get the minimum width required by the cell in the column `column`.
	/// Return 0 if the cell does not exist in this row
	pub fn get_cell_width(&self, column: usize) -> usize {
		return match self.cells.get(column) {
			Some(cell) => cell.get_width(),
			None => 0
		}
	}
	
	/// Get the cell at index `idx`
	pub fn get_cell(&self, idx: usize) -> Option<&Cell> {
		return self.cells.get(idx);
	}
	
	/// Set the `cell` in the row at the given `column`
	pub fn set_cell(&mut self, cell: Cell, column: usize) -> Result<(), &str> {
		if column >= self.len() {
			return Err("Column index is higher than expected");
		}
		self.cells[column] = cell;
		return Ok(());
	}
	
	/// Print the row to `out`, with `separator` as column separator, and `col_width`
	/// specifying the width of each columns
	pub fn print<T: Write>(&self, out: &mut T, separator: char, col_width: &[usize]) -> Result<(), Error> {
		for i in 0..self.get_height() {
			try!(out.write_all(separator.to_string().as_bytes()));
			for j in 0..col_width.len() {
				match self.get_cell(j) {
					Some(ref c) => try!(c.print(out, i, col_width[j])),
					None => try!(Cell::default().print(out, i, col_width[j]))
				};
				try!(out.write_all(separator.to_string().as_bytes()));
			}
			try!(out.write_all(LINEFEED));
		}
		return Ok(());
	}
}

/// This macro simplifies `Row` creation
/// 
/// # Example
/// ```
/// # #[macro_use] extern crate prettytable;
/// # fn main() {
/// let row = row!["Element 1", "Element 2", "Element 3"];
/// // Do something with row
/// # drop(row);
/// # }
/// ```
#[macro_export]
macro_rules! row {
	($($value: expr), *) => (
		$crate::row::Row::new(vec![$($crate::cell::Cell::new(&$value.to_string())), *]);
	);
}