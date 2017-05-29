//! Define table formatting utilities

use std::io::{Write, Error};

use encode_unicode::Utf8Char;

use super::utils::NEWLINE;

/// Alignment for cell's content
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Alignment {
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
#[derive(Clone, Debug, Copy)]
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
		LineSeparator{line: line, junc: junc, ljunc: ljunc, rjunc: rjunc}
	}

	/// Print a full line separator to `out`. `col_width` is a slice containing the width of each column
	pub fn print<T: Write+?Sized>(&self, out: &mut T, col_width: &[usize], colsep: bool, lborder: bool, rborder: bool) -> Result<(), Error> {
		if lborder {
			try!(out.write_all(Utf8Char::from(self.ljunc).as_bytes()));
		}
		let mut iter = col_width.into_iter().peekable();
		while let Some(width) = iter.next() {
			for _ in 0..width+2 {
				try!(out.write_all(Utf8Char::from(self.line).as_bytes()));
			}
			if colsep && iter.peek().is_some() {
				try!(out.write_all(Utf8Char::from(self.junc).as_bytes()));
			}
		}
		if rborder {
			try!(out.write_all(Utf8Char::from(self.rjunc).as_bytes()));
		}
		out.write_all(NEWLINE)
	}
}

impl Default for LineSeparator {
	fn default() -> Self {
		LineSeparator::new('-', '+', '+', '+')
	}
}

/// Contains the table formatting rules
#[derive(Clone, Debug, Copy)]
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
	bottom_sep: Option<LineSeparator>,
	/// Left padding
	pad_left: usize,
	/// Right padding
	pad_right: usize
}

impl TableFormat {

	/// Create a new empty TableFormat.
	pub fn new() -> TableFormat {
		 TableFormat{
			 csep: None,
			 lborder: None,
			 rborder: None,
			 lsep: None,
			 tsep: None,
			 top_sep: None,
			 bottom_sep: None,
			 pad_left: 0,
			 pad_right: 0
		 }
	}

	/// Return a tuple with left and right padding
	pub fn get_padding(&self) -> (usize, usize) {
		(self.pad_left, self.pad_right)
	}

	/// Set left and right padding
	pub fn padding(&mut self, left: usize, right: usize) {
		self.pad_left = left;
		self.pad_right = right;
	}

	/// Set the character used for internal column separation
	pub fn column_separator(&mut self, separator: char) {
		self.csep = Some(separator);
	}

	/// Set the character used for table borders
	pub fn borders(&mut self, border: char) {
		self.lborder = Some(border);
		self.rborder = Some(border);
	}

	/// Set a line separator
	pub fn separator(&mut self, what: LinePosition, separator: LineSeparator) {
		*match what {
			LinePosition::Top => &mut self.top_sep,
			LinePosition::Bottom => &mut self.bottom_sep,
			LinePosition::Title => &mut self.tsep,
			LinePosition::Intern => &mut self.lsep
		} = Some(separator);
	}

	/// Set format for multiple kind of line separator
	pub fn separators(&mut self, what: &[LinePosition], separator: LineSeparator) {
		for pos in what {
			self.separator(*pos, separator);
		}
	}

	fn get_sep_for_line(&self, pos: LinePosition) -> &Option<LineSeparator> {
		match pos {
			LinePosition::Intern => return &self.lsep,
			LinePosition::Top => return &self.top_sep,
			LinePosition::Bottom => return &self.bottom_sep,
			LinePosition::Title => match &self.tsep {
				s @ &Some(_) => s,
				&None => &self.lsep
			}
		}
	}

	/// Print a full line separator to `out`. `col_width` is a slice containing the width of each column
	pub fn print_line_separator<T: Write+?Sized>(&self, out: &mut T, col_width: &[usize], pos: LinePosition) -> Result<(), Error> {
		match *self.get_sep_for_line(pos) {
			Some(ref l) => l.print(out, col_width, self.csep.is_some(), self.lborder.is_some(), self.rborder.is_some()),
			None => Ok(())
		}
	}

	pub fn get_column_separator(&self, pos: ColumnPosition) -> Option<char> {
		match pos {
			ColumnPosition::Left => self.lborder,
			ColumnPosition::Intern => self.csep,
			ColumnPosition::Right => self.rborder
		}
	}

	/// Print a column separator or a table border
	pub fn print_column_separator<T: Write+?Sized>(&self, out: &mut T, pos: ColumnPosition) -> Result<(), Error> {
		match self.get_column_separator(pos) {
			Some(s) => out.write_all(Utf8Char::from(s).as_bytes()),
			None => Ok(())
		}
	}
}

impl Default for TableFormat {
	fn default() -> Self {
		TableFormat::new()
	}
}

/// A builder to create a `TableFormat`
pub struct FormatBuilder {
	format: Box<TableFormat>
}

impl FormatBuilder {
	pub fn new() -> FormatBuilder {
		FormatBuilder {
			format: Box::new(TableFormat::new())
		}
	}

	/// Set left and right padding
	pub fn padding(mut self, left: usize, right: usize) -> Self {
		self.format.padding(left, right);
		self
	}

	/// Set the character used for internal column separation
	pub fn column_separator(mut self, separator: char) -> Self {
		self.format.column_separator(separator);
		self
	}

	/// Set the character used for table borders
	pub fn borders(mut self, border: char) -> Self {
		self.format.borders(border);
		self
	}

	/// Set a line separator format
	pub fn separator(mut self, what: LinePosition, separator: LineSeparator) -> Self {
		self.format.separator(what, separator);
		self
	}

	/// Set separator format for multiple kind of line separators
	pub fn separators(mut self, what: &[LinePosition], separator: LineSeparator) -> Self {
		self.format.separators(what, separator);
		self
	}

	/// Consume this builder and return the generated `TableFormat`
	pub fn build(self) -> TableFormat {
		*self.format
	}
}

/// Predifined formats. Those constants are lazily evaluated when
/// the corresponding struct is dereferenced
pub mod consts {
	use super::{TableFormat, LineSeparator, FormatBuilder, LinePosition};

	lazy_static! {
		/// A line separator made of `-` and `+`
		static ref MINUS_PLUS_SEP: LineSeparator = LineSeparator::new('-', '+', '+', '+');
		/// A line separator made of `=` and `+`
		static ref EQU_PLUS_SEP: LineSeparator = LineSeparator::new('=', '+', '+', '+');

		/// Default table format
		///
		/// # Example
		/// ```text
		/// +----+----+
		/// | T1 | T2 |
		/// +====+====+
		/// | a  | b  |
		/// +----+----+
		/// | d  | c  |
		/// +----+----+
		/// ```
		pub static ref FORMAT_DEFAULT: TableFormat = FormatBuilder::new()
																	.column_separator('|')
																	.borders('|')
																	.separator(LinePosition::Intern, *MINUS_PLUS_SEP)
																	.separator(LinePosition::Title, *EQU_PLUS_SEP)
																	.separator(LinePosition::Bottom, *MINUS_PLUS_SEP)
																	.separator(LinePosition::Top, *MINUS_PLUS_SEP)
																	.padding(1, 1)
																	.build();

		/// Similar to `FORMAT_DEFAULT` but without special separator after title line
		///
		/// # Example
		/// ```text
		/// +----+----+
		/// | T1 | T2 |
		/// +----+----+
		/// | a  | b  |
		/// +----+----+
		/// | c  | d  |
		/// +----+----+
		/// ```
		pub static ref FORMAT_NO_TITLE: TableFormat = FormatBuilder::new()
																	.column_separator('|')
																	.borders('|')
																	.separator(LinePosition::Intern, *MINUS_PLUS_SEP)
																	.separator(LinePosition::Title, *MINUS_PLUS_SEP)
																	.separator(LinePosition::Bottom, *MINUS_PLUS_SEP)
																	.separator(LinePosition::Top, *MINUS_PLUS_SEP)
																	.padding(1, 1)
																	.build();

		/// With no line separator, but with title separator
		///
		/// # Example
		/// ```text
		/// +----+----+
		/// | T1 | T2 |
		/// +----+----+
		/// | a  | b  |
		/// | c  | d  |
		/// +----+----+
		/// ```
		pub static ref FORMAT_NO_LINESEP_WITH_TITLE: TableFormat = FormatBuilder::new()
																	.column_separator('|')
																	.borders('|')
																	.separator(LinePosition::Title, *MINUS_PLUS_SEP)
																	.separator(LinePosition::Bottom, *MINUS_PLUS_SEP)
																	.separator(LinePosition::Top, *MINUS_PLUS_SEP)
																	.padding(1, 1)
																	.build();

		/// With no line or title separator
		///
		/// # Example
		/// ```text
		/// +----+----+
		/// | T1 | T2 |
		/// | a  | b  |
		/// | c  | d  |
		/// +----+----+
		/// ```
		pub static ref FORMAT_NO_LINESEP: TableFormat = FormatBuilder::new()
																	.column_separator('|')
																	.borders('|')
																	.separator(LinePosition::Bottom, *MINUS_PLUS_SEP)
																	.separator(LinePosition::Top, *MINUS_PLUS_SEP)
																	.padding(1, 1)
																	.build();

		/// No column separator
		///
		/// # Example
		/// ```text
		/// --------
		///  T1  T2
		/// ========
		///  a   b
		/// --------
		///  d   c
		/// --------
		/// ```
		pub static ref FORMAT_NO_COLSEP: TableFormat = FormatBuilder::new()
																	.separator(LinePosition::Intern, *MINUS_PLUS_SEP)
																	.separator(LinePosition::Title, *EQU_PLUS_SEP)
																	.separator(LinePosition::Bottom, *MINUS_PLUS_SEP)
																	.separator(LinePosition::Top, *MINUS_PLUS_SEP)
																	.padding(1, 1)
																	.build();

		/// Format for printing a table without any separators (only alignment)
		///
		/// # Example
		/// ```text
		///  T1  T2
		///  a   b
		///  d   c
		/// ```
		pub static ref FORMAT_CLEAN: TableFormat = FormatBuilder::new()
																	.padding(1, 1)
																	.build();

		/// Format for a table with only external borders and title separator
		///
		/// # Example
		/// ```text
		/// +--------+
		/// | T1  T2 |
		/// +========+
		/// | a   b  |
		/// | c   d  |
		/// +--------+
		/// ```
		pub static ref FORMAT_BORDERS_ONLY: TableFormat = FormatBuilder::new()
																	.padding(1, 1)
																	.separator(LinePosition::Title, *EQU_PLUS_SEP)
																	.separator(LinePosition::Bottom, *MINUS_PLUS_SEP)
																	.separator(LinePosition::Top, *MINUS_PLUS_SEP)
																	.borders('|')
																	.build();

		/// A table with no external border
		///
		/// # Example
		/// ```text
		///  T1 | T2
		/// ====+====
		///  a  | b
		/// ----+----
		///  c  | d
		/// ```
		pub static ref FORMAT_NO_BORDER: TableFormat = FormatBuilder::new()
																	.padding(1, 1)
																	.separator(LinePosition::Intern, *MINUS_PLUS_SEP)
																	.separator(LinePosition::Title, *EQU_PLUS_SEP)
																	.column_separator('|')
																	.build();

		/// A table with no external border and no line separation
		///
		/// # Example
		/// ```text
		///  T1 | T2
		/// ----+----
		///  a  | b
		///  c  | d
		/// ```
		pub static ref FORMAT_NO_BORDER_LINE_SEPARATOR: TableFormat = FormatBuilder::new()
																	.padding(1, 1)
																	.separator(LinePosition::Title, *MINUS_PLUS_SEP)
																	.column_separator('|')
																	.build();
	}
}
