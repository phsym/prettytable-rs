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
	pub fn print<T: Write+?Sized>(&self, out: &mut T, col_width: &[usize], with_colsep: bool) -> Result<(), Error> {
		if with_colsep {
			try!(out.write_all(&self.cross));
		}
		for width in col_width {
			try!(out.write_all(&vec![self.line[0]; width+2]));
			if with_colsep {
				try!(out.write_all(&self.cross));
			}
		}
		return out.write_all(NEWLINE);
	}
}

/// Contains the table formatting rules
#[derive(Clone, Debug)]
pub struct TableFormat {
	col_sep: Option<[u8; 1]>,
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
	pub fn new(col_sep: Option<char>, line_sep: Option<LineSeparator>, title_sep: Option<LineSeparator>) -> TableFormat {
		 let csep = match col_sep {
		 	Some(c) => Some([c as u8]),
		 	None => None
		 };
		 return TableFormat{col_sep: csep, line_sep: line_sep, title_sep: title_sep};
	}
	
	/// Print a full line separator to `out`. `col_width` is a slice containing the width of each column
	pub fn print_line_separator<T: Write+?Sized>(&self, out: &mut T, col_width: &[usize]) -> Result<(), Error> {
		if let Some(ref l) = self.line_sep {
			return l.print(out, col_width, self.col_sep.is_some());
		}
		return Ok(());
	}
	
	/// Print a full title separator to `out`. `col_width` is a slice containing the width of each column
	pub fn print_title_separator<T: Write+?Sized>(&self, out: &mut T, col_width: &[usize]) -> Result<(), Error> {
		if let Some(ref l) = self.title_sep {
			return l.print(out, col_width, self.col_sep.is_some());
		}
		return self.print_line_separator(out, col_width);
	}
	
	/// Print a column separator to `out`
	pub fn print_column_separator<T: Write+?Sized>(&self, out: &mut T) -> Result<(), Error> {
		return match self.col_sep {
			Some(ref s) => out.write_all(s),
			None => Ok(())
		};
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
pub const FORMAT_DEFAULT: TableFormat = TableFormat{col_sep: Some(['|' as u8]), line_sep: Some(MINUS_PLUS_SEP), title_sep: Some(EQU_PLUS_SEP)};

/// Similar to `FORMAT_DEFAULT` but without special separator after title line
pub const FORMAT_NO_TITLE: TableFormat = TableFormat{col_sep: Some(['|' as u8]), line_sep: Some(MINUS_PLUS_SEP), title_sep: Some(MINUS_PLUS_SEP)};

/// With no line separator, but with title separator
pub const FORMAT_NO_LINESEP_WITH_TITLE: TableFormat = TableFormat{col_sep: Some(['|' as u8]), line_sep: None, title_sep: Some(MINUS_PLUS_SEP)};

/// With no line or title separator
pub const FORMAT_NO_LINESEP: TableFormat = TableFormat{col_sep: Some(['|' as u8]), line_sep: None, title_sep: None};

/// No column seprarator
pub const FORMAT_NO_COLSEP: TableFormat = TableFormat{col_sep: None, line_sep: Some(MINUS_PLUS_SEP), title_sep: Some(EQU_PLUS_SEP)};

/// Format for printing a table without any separators (only alignment)
pub const FORMAT_NO_BORDER: TableFormat = TableFormat{col_sep: None, line_sep: None, title_sep: None};
