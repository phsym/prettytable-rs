//! Define table formatting utilities

use std::io::{Write, Error};

use super::utils::NEWLINE;

/// Alignment for cell's content
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Alignment  {
	LEFT,
	CENTER,
	RIGHT
}

/// Position of a line separator in a table
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum LinePosition {
	Top,
	Title,
	Intern,
	Bottom
}

/// Position of a column separator in a row
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum ColumnPosition {
	Left,
	Intern,
	Right
}

/// Contains the character used for printing a line separator
#[derive(Clone, Debug)]
pub struct LineSeparator {
	/// Line separator
	line: char,
	/// Internal junction separator
	junc: char,
	/// Left junction separator
	ljunc: char,
	/// Right junction separator
	rjunc: char
}

impl LineSeparator {

	/// Create a new line separator instance where `line` is the character used to separate 2 lines
	/// and `junc` is the one used for junctions between columns and lines
	pub fn new(line: char, junc: char, ljunc: char, rjunc: char) -> LineSeparator {
		return LineSeparator{line: line, junc: junc, ljunc: ljunc, rjunc: rjunc};
	}

	/// Print a full line separator to `out`. `col_width` is a slice containing the width of each column
	pub fn print<T: Write+?Sized>(&self, out: &mut T, col_width: &[usize], colsep: bool, lborder: bool, rborder: bool) -> Result<(), Error> {
		if lborder {
			try!(out.write_all(&[self.ljunc as u8]));
		}
		let mut iter = col_width.into_iter().peekable();
		while let Some(width) = iter.next() {
			try!(out.write_all(&vec![self.line as u8; width+2]));
			if colsep && iter.peek().is_some() {
				try!(out.write_all(&[self.junc as u8]));
			}
		}
		if rborder {
			try!(out.write_all(&[self.rjunc as u8]));
		}
		return out.write_all(NEWLINE);
	}
}

impl Default for LineSeparator {
	fn default() -> Self {
		return LineSeparator::new('-', '+', '+', '+');
	}
}

/// Contains the table formatting rules
#[derive(Clone, Debug)]
pub struct TableFormat {
	/// Optional column separator character
	csep: Option<char>,
	/// Optional left border character
	lborder: Option<char>,
	/// Optional right border character
	rborder: Option<char>,
	/// Optional internal line separator
	lsep: Option<LineSeparator>,
	/// Optional title line separator
	tsep: Option<LineSeparator>,
	/// Optional top line separator
	top_sep: Option<LineSeparator>,
	/// Optional bottom line separator
	bottom_sep: Option<LineSeparator>
}

impl Default for TableFormat {
	fn default() -> Self {
		return TableFormat::new(None, None, None);
	}
}

impl TableFormat {

	/// Create a new TableFormat.
	///
	/// `csep` is the character used for separating columns.
	/// `lsep` is an optional `LineSeparator` defining how to separate lines.
	/// `tsep` is an optional `LineSeparator` defining the format of the separator after the title line (if set).
	/// If `tsep` is set to `None`, then `lsep` will be used, f it's set.
	pub fn new(csep: Option<char>, lsep: Option<LineSeparator>, tsep: Option<LineSeparator>) -> TableFormat {
		 return TableFormat{
			 csep: csep,
			 lborder: None,
			 rborder: None,
			 lsep: lsep,
			 tsep: tsep,
			 top_sep: None,
			 bottom_sep: None
		 };
	}

	fn get_sep_for_line(&self, pos: LinePosition) -> &Option<LineSeparator> {
		return match pos {
			LinePosition::Intern => return &self.lsep,
			LinePosition::Top => return &self.top_sep,
			LinePosition::Bottom => return &self.bottom_sep,
			LinePosition::Title => match &self.tsep {
				s @ &Some(_) => s,
				&None => &self.lsep
			}
		};
	}

	/// Print a full line separator to `out`. `col_width` is a slice containing the width of each column
	pub fn print_line_separator<T: Write+?Sized>(&self, out: &mut T, col_width: &[usize], pos: LinePosition) -> Result<(), Error> {
		return match *self.get_sep_for_line(pos) {
			Some(ref l) => l.print(out, col_width, self.csep.is_some(), self.lborder.is_some(), self.rborder.is_some()),
			None => Ok(())
		};
	}

	/// Print a column separator
	fn get_column_separator(&self, pos: ColumnPosition) -> Option<char> {
		return match pos {
			ColumnPosition::Left => self.lborder,
			ColumnPosition::Intern => self.csep,
			ColumnPosition::Right => self.rborder
		};
	}

	pub fn print_column_separator<T: Write+?Sized>(&self, out: &mut T, pos: ColumnPosition) -> Result<(), Error> {
		return match self.get_column_separator(pos) {
			Some(s) => out.write_all(&[s as u8]),
			None => Ok(())
		};
	}
}

/// A line separator mad of `-` and `+`
pub const MINUS_PLUS_SEP: LineSeparator = LineSeparator{line: '-', junc: '+', ljunc: '+', rjunc: '+'};
/// A line separator mad of `=` and `+`
pub const EQU_PLUS_SEP: LineSeparator = LineSeparator{line: '=', junc: '+', ljunc: '+', rjunc: '+'};

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
pub const FORMAT_DEFAULT: TableFormat = TableFormat{csep: Some('|'), lborder: Some('|'), rborder: Some('|'), lsep: Some(MINUS_PLUS_SEP), tsep: Some(EQU_PLUS_SEP), top_sep: Some(MINUS_PLUS_SEP), bottom_sep: Some(MINUS_PLUS_SEP)};

/// Similar to `FORMAT_DEFAULT` but without special separator after title line
pub const FORMAT_NO_TITLE: TableFormat = TableFormat{csep: Some('|'), lborder: Some('|'), rborder: Some('|'), lsep: Some(MINUS_PLUS_SEP), tsep: Some(MINUS_PLUS_SEP), top_sep: Some(MINUS_PLUS_SEP), bottom_sep: Some(MINUS_PLUS_SEP)};

/// With no line separator, but with title separator
pub const FORMAT_NO_LINESEP_WITH_TITLE: TableFormat = TableFormat{csep: Some('|'), lborder: Some('|'), rborder: Some('|'), lsep: None, tsep: Some(MINUS_PLUS_SEP), top_sep: Some(MINUS_PLUS_SEP), bottom_sep: Some(MINUS_PLUS_SEP)};

/// With no line or title separator
pub const FORMAT_NO_LINESEP: TableFormat = TableFormat{csep: Some('|'), lborder: Some('|'), rborder: Some('|'), lsep: None, tsep: None, top_sep: None, bottom_sep: None};

/// No column seprarator
pub const FORMAT_NO_COLSEP: TableFormat = TableFormat{csep: None, lborder: None, rborder: None, lsep: Some(MINUS_PLUS_SEP), tsep: Some(EQU_PLUS_SEP), top_sep: Some(MINUS_PLUS_SEP), bottom_sep: Some(MINUS_PLUS_SEP)};

/// Format for printing a table without any separators (only alignment)
pub const FORMAT_NO_BORDER: TableFormat = TableFormat{csep: None, lborder: None, rborder: None, lsep: None, tsep: None, top_sep: None, bottom_sep: None};
