//! This module contains definition of table/row cells stuff

use std::io::{Write, Error};
use std::string::ToString;
use unicode_width::UnicodeWidthStr;
use term::{Attr, Terminal, color};
use super::format::Align;

/// Represent a table cell containing a string.
///
/// Once created, a cell's content cannot be modified.
/// The cell would have to be replaced by another one
#[derive(Clone, Debug)]
pub struct Cell {
	content: Vec<String>,
	width: usize,
	align: Align,
	style: Vec<Attr>
}

impl Cell {
	/// Create a new `Cell` initialized with content from `string`.
	/// Text alignment in cell is configurable with the `align` argument
	pub fn new_align(string: &str, align: Align) -> Cell {
		let content: Vec<String> = string.lines().map(|ref x| x.to_string()).collect();
		let mut width = 0;
		for cont in &content {
			let l = UnicodeWidthStr::width(&cont[..]);
			if l > width {
				width = l;
			}
		}
		return Cell {
			content: content,
			width: width,
			align: align,
			style: Vec::new()
		};
	}
	
	/// Create a new `Cell` initialized with content from `string`.
	/// By default, content is align to `LEFT`
	pub fn new(string: &str) -> Cell {
		return Cell::new_align(string, Align::LEFT);
	}
	
	/// Set text alignment in the cell
	pub fn align(&mut self, align: Align) {
		self.align = align;
	}
	
	/// Add a style attribute to the cell
	pub fn style(&mut self, attr: Attr) {
		self.style.push(attr);
	}
	
	/// Add a style attribute to the cell. Can be chained
	pub fn with_style(mut self, attr: Attr) -> Cell {
		self.style(attr);
		return self;
	}
	
	/// Remove all style attributes and reset alignment to default (LEFT)
	pub fn reset_style(&mut self) {
		self.style.clear();
		self.align(Align::LEFT);
	}
	
	/// Set the cell's style by applying the given specifier string
	///
	/// # Style spec syntax
	///
	/// The syntax for the style specifier looks like this :
	/// **FrBybl** which means **F**oreground **r**ed **B**ackground **y**ellow **b**old **l**eft
	///
	/// ### List of supported specifiers :
	///
	/// * **F** : **F**oreground (must be followed by a color specifier)
	/// * **B** : **B**ackground (must be followed by a color specifier)
	/// * **b** : **b**old
	/// * **i** : **i**talic
	/// * **u** : **u**nderline
	/// * **c** : Align **c**enter
	/// * **l** : Align **l**eft
	/// * **r** : Align **r**ight
	/// * **d** : **d**efault style
	/// 
	/// ### List of color specifiers :
	///
	/// * **r** : Red
	/// * **b** : Blue
	/// * **g** : Green
	/// * **y** : Yellow
	/// * **c** : Cyan
	/// * **m** : Magenta
	/// * **w** : White
	/// * **d** : Black
	/// 
	/// And capital letters are for **bright** colors.
	/// Eg :
	///
	/// * **R** : Bright Red
	/// * **B** : Bright Blue
	/// * ... and so on ...
	pub fn style_spec(mut self, spec: &str) -> Cell {
		let mut foreground = false;
		let mut background = false;
		for c in spec.chars() {
			if foreground || background {
				let color = match c {
					'r' => color::RED,
					'R' => color::BRIGHT_RED,
					'b' => color::BLUE,
					'B' => color::BRIGHT_BLUE,
					'g' => color::GREEN,
					'G' => color::BRIGHT_GREEN,
					'y' => color::YELLOW,
					'Y' => color::BRIGHT_YELLOW,
					'c' => color::CYAN,
					'C' => color::BRIGHT_CYAN,
					'm' => color::MAGENTA,
					'M' => color::BRIGHT_MAGENTA,
					'w' => color::WHITE,
					'W' => color::BRIGHT_WHITE,
					'd' => color::BLACK,
					'D' => color::BRIGHT_BLACK,
					_ => { // Silently ignore unknown tags
							foreground = false;
							background = false;
							continue;
						 }
				};
				if foreground { self.style(Attr::ForegroundColor(color)); }
				else if background { self.style(Attr::BackgroundColor(color)); }
				foreground = false;
				background = false;
			}
			else {
				match c {
					'F' => foreground = true,
					'B' => background = true,
					'b' => self.style(Attr::Bold),
					'i' => self.style(Attr::Italic(true)),
					'u' => self.style(Attr::Underline(true)),
					'c' => self.align(Align::CENTER),
					'l' => self.align(Align::LEFT),
					'r' => self.align(Align::RIGHT),
					'd' => {/* Default : do nothing */}
					_ => {/* Silently ignore unknown tags */}
				}
			}
		}
		return self;
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
	pub fn print<T: Write+?Sized>(&self, out: &mut T, idx: usize, col_width: usize) -> Result<(), Error> {
		let c = match self.content.get(idx) {
			Some(s) => s.as_ref(),
			None => ""
		};
		return match self.align {
			Align::LEFT   => write!(out, " {: <1$} ", c, col_width),
			Align::CENTER => write!(out, " {: ^1$} ", c, col_width),
			Align::RIGHT  => write!(out, " {: >1$} ", c, col_width),
		}
	}
	
	/// Apply style then call `print` to print the cell into a terminal
	pub fn print_term<T: Terminal+?Sized>(&self, out: &mut T, idx: usize, col_width: usize) -> Result<(), Error> {
		for a in &self.style {
			try!(out.attr(a.clone()));
		}
		try!(self.print(out, idx, col_width));
		try!(out.reset());
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
	/// Return a cell initialized with a single empty `String`, with LEFT alignment
	fn default() -> Cell {
		return Cell {
			content: vec!["".to_string(); 1],
			width: 0,
			align: Align::LEFT,
			style: Vec::new()
		};
	}
}

/// This macro simplifies `Cell` creation
/// 
/// Support 2 syntax : With and without style specification.
/// # Syntax
/// ```text
/// cell!(value);
/// ```
/// or
/// 
/// ```text
/// cell!(spec:value);
/// ```
/// Value must implement the `std::string::ToString` trait
///
/// For details about style specifier syntax, check doc for [Cell::style_spec](cell/struct.Cell.html#method.style_spec) method
/// # Example
/// ```
/// # #[macro_use] extern crate prettytable;
/// # fn main() {
/// let cell = cell!("value");
/// // Do something with the cell
/// # drop(cell);
/// // Create a cell with style (Red foreground, Bold, aligned to left);
/// let styled = cell!(Frbl:"value");
/// # drop(styled);
/// # }
/// ```
#[macro_export]
macro_rules! cell {
	() => ($crate::cell::Cell::default());
	($value:expr) => ($crate::cell::Cell::new(&$value.to_string()));
	($style:ident : $value:expr) => (cell!($value).style_spec(stringify!($style)));
}

#[cfg(test)]
mod tests {
	use cell::Cell;
	use utils::StringWriter;
	use format::Align;

	#[test]
	fn ascii() {
		let ascii_cell = Cell::new("hello");
		assert_eq!(ascii_cell.get_width(), 5);

		let mut out = StringWriter::new();
		let _ = ascii_cell.print(&mut out, 0, 10);
		assert_eq!(out.as_string(), " hello      ");
	}

	#[test]
	fn unicode() {
		let unicode_cell = Cell::new("привет");
		assert_eq!(unicode_cell.get_width(), 6);

		let mut out = StringWriter::new();
		let _ = unicode_cell.print(&mut out, 0, 10);
		assert_eq!(out.as_string(), " привет     ");
	}
	
	#[test]
	fn align_left() {
		let cell = Cell::new_align("test", Align::LEFT);
		let mut out = StringWriter::new();
		let _ = cell.print(&mut out, 0, 10);
		assert_eq!(out.as_string(), " test       ");
	}
	
	#[test]
	fn align_center() {
		let cell = Cell::new_align("test", Align::CENTER);
		let mut out = StringWriter::new();
		let _ = cell.print(&mut out, 0, 10);
		assert_eq!(out.as_string(), "    test    ");
	}
	
	#[test]
	fn align_right() {
		let cell = Cell::new_align("test", Align::RIGHT);
		let mut out = StringWriter::new();
		let _ = cell.print(&mut out, 0, 10);
		assert_eq!(out.as_string(), "       test ");
	}
}

