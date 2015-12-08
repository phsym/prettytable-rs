//! A formatted and aligned table printer written in rust
extern crate unicode_width;
extern crate term;

use std::io;
use std::io::{Write, Error};
use std::fmt;
use std::iter::{FromIterator, IntoIterator};
use std::ops::{Index, IndexMut};

use term::{Terminal, stdout};

pub mod cell;
pub mod row;
pub mod format;
mod utils;

use row::Row;
use cell::Cell;
use format::{TableFormat, FORMAT_DEFAULT};
use utils::StringWriter;

/// A Struct representing a printable table
#[derive(Clone, Debug)]
pub struct Table {
	format: TableFormat,
	titles: Option<Row>,
	rows: Vec<Row>
}

impl Table {
	/// Create an empty table
	pub fn new() -> Table {
		return Self::init(Vec::new());
	}
	
	/// Create a table initialized with `rows`
	pub fn init(rows: Vec<Row>) -> Table {
		return Table {
			rows: rows,
			titles: None,
			format: FORMAT_DEFAULT
		};
	}
	
	/// Change separators the table format
	pub fn set_format(&mut self, format: TableFormat) {
		self.format = format;
	}
	
	/// Compute and return the number of column
	pub fn get_column_num(&self) -> usize {
		let mut cnum = 0;
		for r in &self.rows {
			let l = r.len();
			if l > cnum {
				cnum = l;
			}
		}
		return cnum;
	}
	
	/// Get the number of rows
	pub fn len(&self) -> usize {
		return self.rows.len();
	}
	
	/// Set the optional title lines
	pub fn set_titles(&mut self, titles: Row) {
		self.titles = Some(titles);
	}
	
	/// Unset the title line
	pub fn unset_titles(&mut self) {
		self.titles = None;
	}
	
	/// Get a mutable reference to a row
	pub fn get_mut_row(&mut self, row: usize) -> Option<&mut Row> {
		return self.rows.get_mut(row);
	}
	
	/// Get an immutable reference to a row
	pub fn get_row(&self, row: usize) -> Option<&Row> {
		return self.rows.get(row);
	}
	
	/// Append a row in the table, transferring ownership of this row to the table
	/// and returning a mutable reference to the row
	pub fn add_row(&mut self, row: Row) -> &mut Row {
		self.rows.push(row);
		let l = self.rows.len()-1;
		return &mut self.rows[l];
	}
	
	/// Append an empty row in the table. Return a mutable reference to this new row.
	pub fn add_empty_row(&mut self) -> &mut Row {
		return self.add_row(Row::default());	
	}
	
	/// Insert `row` at the position `index`, and return a mutable reference to this row.
	/// If index is higher than current numbers of rows, `row` is appended at the end of the table
	pub fn insert_row(&mut self, index: usize, row: Row) -> &mut Row {
		if index < self.rows.len() {
			self.rows.insert(index, row);
			return &mut self.rows[index];
		} else {
			return self.add_row(row);
		}
	}
	
	/// Modify a single element in the table
	pub fn set_element(&mut self, element: &str, column: usize, row: usize) -> Result<(), &str> {
		let rowline = try!(self.get_mut_row(row).ok_or("Cannot find row"));
		// TODO : If a cell already exist, copy it's alignment parameter
		return rowline.set_cell(Cell::new(element), column);
	}
	
	/// Remove the row at position `index`. Silently skip if the row does not exist
	pub fn remove_row(&mut self, index: usize) {
		if index < self.rows.len() {
			self.rows.remove(index);
		}
	}
	
	/// Get the width of the column at position `col_idx`.
	/// Return 0 if the column does not exists;
	pub fn get_column_width(&self, col_idx: usize) -> usize {
		let mut width = match self.titles {
			Some(ref t) => t.get_cell_width(col_idx),
			None => 0
		};
		for r in &self.rows {
			let l = r.get_cell_width(col_idx);
			if l > width {
				width = l;
			}
		}
		return width;
	}
	
	/// Get the width of all columns, and return a slice 
	/// with the result for each column
	pub fn get_all_column_width(&self) -> Vec<usize> {
		let colnum = self.get_column_num();
		let mut col_width = vec![0usize; colnum];
		for i in 0..colnum {
			col_width[i] = self.get_column_width(i);
		}
		return col_width;
	}
	
	/// Return an iterator over the immutable cells of the column specified by `column`
	pub fn column_iter(&self, column: usize) -> ColumnIter {
		return ColumnIter(self.rows.iter(), column);
	}
	
	/// Return an iterator over the mutable cells of the column specified by `column`
	pub fn column_iter_mut(&mut self, column: usize) -> ColumnIterMut {
		return ColumnIterMut(self.rows.iter_mut(), column);
	}
	
	/// Internal only
	fn __print<T: Write+?Sized, F>(&self, out: &mut T, f: F) -> Result<(), Error> where F: Fn(&Row, &mut T, &TableFormat, &[usize]) -> Result<(), Error> {
		// Compute columns width
		let col_width = self.get_all_column_width();
		try!(self.format.print_line_separator(out, &col_width));
		if let Some(ref t) = self.titles {
			try!(f(t, out, &self.format, &col_width));
			try!(self.format.print_title_separator(out, &col_width));
		}
		// Print rows
		for r in &self.rows {
			try!(f(r, out, &self.format, &col_width));
			try!(self.format.print_line_separator(out, &col_width));
		}
		return out.flush();
	}
	
	/// Print the table to `out`
	pub fn print<T: Write+?Sized>(&self, out: &mut T) -> Result<(), Error> {
		return self.__print(out, Row::print);
	}
	
	/// Print the table to terminal `out`, applying styles when needed
	pub fn print_term<T: Terminal+?Sized>(&self, out: &mut T) -> Result<(), Error> {
		return self.__print(out, Row::print_term);
	}
	
	/// Print the table to standard output
	/// # Panic
	/// Panic if writing to standard output fails
	pub fn printstd(&self) {
		match stdout() {
			Some(mut o) => self.print_term(&mut *o),
			None => self.print(&mut io::stdout()),
		}.ok().expect("Cannot print table to standard output");
	}
}

impl Index<usize> for Table {
	type Output = Row;
	fn index(&self, idx: usize) -> &Self::Output {
		return &self.rows[idx];
	}
}

impl IndexMut<usize> for Table {
	fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
		return &mut self.rows[idx];
	}
}

impl fmt::Display for Table {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		let mut writer = StringWriter::new();
		if let Err(_) = self.print(&mut writer) {
			return Err(fmt::Error)
		}
		return fmt.write_str(writer.as_string());
	}
}

impl <B: ToString, A: IntoIterator<Item=B>> FromIterator<A> for Table {
	fn from_iter<T>(iterator: T) -> Table where T: IntoIterator<Item=A> {
		return Self::init(iterator.into_iter().map(|r| Row::from(r)).collect());
	}
}

impl <T, A, B> From<T> for Table where B: ToString, A: IntoIterator<Item=B>, T : IntoIterator<Item=A> {
	fn from(it: T) -> Table {
		return Self::from_iter(it);
	}
}

/// Iterator over immutable cells in a column
pub struct ColumnIter<'a>(std::slice::Iter<'a, Row>, usize);

impl <'a> std::iter::Iterator for ColumnIter<'a> {
	type Item = &'a Cell;
	
	fn next(&mut self) -> Option<&'a Cell> {
		return match self.0.next() {
			None => None,
			Some(row) => row.get_cell(self.1)
		}
	}
}

/// Iterator over mutable cells in a column
pub struct ColumnIterMut<'a>(std::slice::IterMut<'a, Row>, usize);

impl <'a> std::iter::Iterator for ColumnIterMut<'a> {
	type Item = &'a mut Cell;
	
	fn next(&mut self) -> Option<&'a mut Cell> {
		return match self.0.next() {
			None => None,
			Some(row) => row.get_mut_cell(self.1)
		}
	}
}

/// Create a table filled with some values
/// 
/// All the arguments used for elements must implement the `std::string::ToString` trait
/// # Syntax
/// ```text
/// table!([Element1_ row1, Element2_ row1, ...], [Element1_row2, ...], ...);
/// ```
///
/// # Example
/// ```
/// # #[macro_use] extern crate prettytable;
/// # fn main() {
/// // Create a table initialized with some rows :
/// let tab = table!(["Element1", "Element2", "Element3"],
/// 				 [1, 2, 3],
/// 				 ["A", "B", "C"]
/// 				 );
/// # drop(tab);
/// # }
/// ```
/// 
/// Some style can also be given in table creation
/// 
/// ```
/// # #[macro_use] extern crate prettytable;
/// # fn main() {
/// let tab = table!([FrByl:"Element1", Fgc:"Element2", "Element3"],
/// 				 [FrBy -> 1, 2, 3],
/// 				 ["A", "B", "C"]
/// 				 );
/// # drop(tab);
/// # }
/// ```
///
/// For details about style specifier syntax, check doc for [Cell::style_spec](cell/struct.Cell.html#method.style_spec) method
#[macro_export]
macro_rules! table {
	($([$($content:tt)*]), *) => (
		$crate::Table::init(vec![$(row![$($content)*]), *])
	);
}

/// Create a table with `table!` macro, print it to standard output, then return this table for future usage.
/// 
/// The syntax is the same that the one for the `table!` macro
#[macro_export]
macro_rules! ptable {
	($($content:tt)*) => (
		{
			let tab = table!($($content)*);
			tab.printstd();
			tab
		}
	);
}

#[cfg(test)]
mod tests {
	
	use Table;
	use row::Row;
	use cell::Cell;
	
	#[test]
	fn table() {
		let mut table = Table::new();
		table.add_row(Row::new(vec![Cell::new("a"), Cell::new("bc"), Cell::new("def")]));
		table.add_row(Row::new(vec![Cell::new("def"), Cell::new("bc"), Cell::new("a")]));
		table.set_titles(Row::new(vec![Cell::new("t1"), Cell::new("t2"), Cell::new("t3")]));
		let out = "\
+-----+----+-----+
| t1  | t2 | t3  |
+=====+====+=====+
| a   | bc | def |
+-----+----+-----+
| def | bc | a   |
+-----+----+-----+
";
		assert_eq!(table.to_string().replace("\r\n", "\n"), out);
	}
	
	#[test]
	fn index() {
		let mut table = Table::new();
		table.add_row(Row::new(vec![Cell::new("a"), Cell::new("bc"), Cell::new("def")]));
		table.add_row(Row::new(vec![Cell::new("def"), Cell::new("bc"), Cell::new("a")]));
		table.set_titles(Row::new(vec![Cell::new("t1"), Cell::new("t2"), Cell::new("t3")]));
		assert_eq!(table[1][1].get_content(), "bc");
		
		table[1][1] = Cell::new("newval");
		assert_eq!(table[1][1].get_content(), "newval");
		
		let out = "\
+-----+--------+-----+
| t1  | t2     | t3  |
+=====+========+=====+
| a   | bc     | def |
+-----+--------+-----+
| def | newval | a   |
+-----+--------+-----+
";
		assert_eq!(table.to_string().replace("\r\n", "\n"), out);
	}
}
