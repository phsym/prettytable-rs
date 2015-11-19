//! Define table formatting utilities

use std::io::{Write, Error};

use super::utils::NEWLINE;

/// Alignment for cell's content
#[derive(Clone, Debug, PartialEq)]
pub enum Align  {
	LEFT,
	CENTER,
	RIGHT
}

/// Contains the character used for printing a line separator
#[derive(Clone, Debug)]
pub struct LineSeparator {
	line: [u8; 1],
	cross: [u8; 1],
}

impl LineSeparator {
	
	/// Create a new line separator instance where `line` is the character used to separate 2 lines
	/// and `cross` is the one used when column separaors and line separators cross
	pub fn new(line: char, cross: char) -> LineSeparator {
		return LineSeparator{line: [line as u8], cross: [cross as u8]};
	}
	
	/// Print a full line separator to `out`. `col_width` is a slice containing the width of each column
	pub fn print<T: Write+?Sized>(&self, out: &mut T, col_width: &[usize]) -> Result<(), Error> {
		try!(out.write_all(&self.cross));
		for width in col_width {
			try!(out.write_all(&vec![self.line[0]; width+2]));
			try!(out.write_all(&self.cross));
		}
		return out.write_all(NEWLINE);
	}
}

/// Contains the table formatting rules
#[derive(Clone, Debug)]
pub struct TableFormat {
	col_sep: [u8; 1],
	line_sep: Option<LineSeparator>,
	title_sep: Option<LineSeparator>
}

impl TableFormat {
	
	/// Create a new TableFormat.
	///
	/// `col_sep` is the character used for separating columns.
	/// `line_sep` is an optional `LineSeparator` defining how to separate lines.
	/// `title_sep` is an optional `LineSeparator` defining the format of the separator after the title line (if set).
	/// If `title_sep` is set to `None`, then `line_sep` will be used, f it's set.
	pub fn new(col_sep: char, line_sep: Option<LineSeparator>, title_sep: Option<LineSeparator>) -> TableFormat {
		return TableFormat{col_sep: [col_sep as u8], line_sep: line_sep, title_sep: title_sep};
	}
	
	/// Print a full line separator to `out`. `col_width` is a slice containing the width of each column
	pub fn print_line_separator<T: Write+?Sized>(&self, out: &mut T, col_width: &[usize]) -> Result<(), Error> {
		if let Some(ref l) = self.line_sep {
			return l.print(out, col_width);
		}
		return Ok(());
	}
	
	/// Print a full title separator to `out`. `col_width` is a slice containing the width of each column
	pub fn print_title_separator<T: Write+?Sized>(&self, out: &mut T, col_width: &[usize]) -> Result<(), Error> {
		if let Some(ref l) = self.title_sep {
			return l.print(out, col_width);
		}
		return self.print_line_separator(out, col_width);
	}
	
	/// Print a column separator to `out`
	pub fn print_column_separator<T: Write+?Sized>(&self, out: &mut T) -> Result<(), Error> {
		return out.write_all(&self.col_sep);
	}
}

/// A line separator mad of `-` and `+`
pub const MINUS_PLUS_SEP: LineSeparator = LineSeparator{line: ['-' as u8], cross: ['+' as u8]};
/// A line separator mad of `=` and `+`
pub const EQU_PLUS_SEP: LineSeparator = LineSeparator{line: ['=' as u8], cross: ['+' as u8]};

/// Default table format, printing a table like this :
///
/// ```text
/// +----+----+
/// | T1 | T2 |
/// +====+====+
/// |    |    |
/// +----+----+
/// |    |    |
/// +----+----+
/// ```
pub const FORMAT_DEFAULT: TableFormat = TableFormat{col_sep: ['|' as u8], line_sep: Some(MINUS_PLUS_SEP), title_sep: Some(EQU_PLUS_SEP)};

/// Similar to `FORMAT_DEFAULT` but without special separator after title line
pub const FORMAT_NO_LINESEP: TableFormat = TableFormat{col_sep: ['|' as u8], line_sep: None, title_sep: Some(MINUS_PLUS_SEP)};

/// Format for printing a table without any separators (only alignment)
pub const FORMAT_BLANK: TableFormat = TableFormat{col_sep: [' ' as u8], line_sep: None, title_sep: None};