//! This module contains definition of table rows stuff
use std::io::{Write, Error};
use std::iter::FromIterator;

use super::utils::NEWLINE;
use super::cell::Cell;
use super::format::TableFormat;

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
	pub fn empty() -> Row {
		return Self::new(vec![Cell::default(); 0]);
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
	
	/// Get the mutable cell at index `idx`
	pub fn get_mut_cell(&mut self, idx: usize) -> Option<&mut Cell> {
		return self.cells.get_mut(idx);
	}
	
	/// Set the `cell` in the row at the given `column`
	pub fn set_cell(&mut self, cell: Cell, column: usize) -> Result<(), &str> {
		if column >= self.len() {
			return Err("Cannot find cell");
		}
		self.cells[column] = cell;
		return Ok(());
	}
	
	/// Append a `cell` at the end of the row
	pub fn add_cell(&mut self, cell: Cell) {
		self.cells.push(cell);
	}
	
	/// Insert `cell` at position `index`. If `index` is higher than the row lenght,
	/// the cell will be appended at the end
	pub fn insert_cell(&mut self, index: usize, cell: Cell) {
		if index < self.cells.len() {
			self.cells.insert(index, cell);
		} else {
			self.add_cell(cell);
		}
	}
	
	/// Remove the cell at position `index`. Silently skip if this cell does not exist
	pub fn remove_cell(&mut self, index: usize) {
		if index < self.cells.len() {
			self.cells.remove(index);
		}
	}
	
	/// Print the row to `out`, with `separator` as column separator, and `col_width`
	/// specifying the width of each columns
	pub fn print<T: Write>(&self, out: &mut T, format: &TableFormat, col_width: &[usize]) -> Result<(), Error> {
		for i in 0..self.get_height() {
			try!(format.print_column_separator(out));
			for j in 0..col_width.len() {
				match self.get_cell(j) {
					Some(ref c) => try!(c.print(out, i, col_width[j])),
					None => try!(Cell::default().print(out, i, col_width[j]))
				};
				try!(format.print_column_separator(out));
			}
			try!(out.write_all(NEWLINE));
		}
		return Ok(());
	}
}

impl Default for Row {
	fn default() -> Row {
		return Row::empty();
	}
}

impl <A: ToString> FromIterator<A> for Row {
	fn from_iter<T>(iterator: T) -> Row where T: IntoIterator<Item=A> {
		return Self::new(iterator.into_iter().map(|ref e| Cell::from(e)).collect());
	}
}

impl <T, A> From<T> for Row where A: ToString, T : IntoIterator<Item=A> {
	fn from(it: T) -> Row {
		return Self::from_iter(it);
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