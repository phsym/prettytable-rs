//! This module contains definition of table rows stuff
use std::io::{Write, Error};
use std::iter::FromIterator;
use std::slice::{Iter, IterMut};
use std::ops::{Index, IndexMut};

use term::Terminal;

use super::utils::NEWLINE;
use super::cell::Cell;
use super::format::{TableFormat, ColumnPosition};

/// Represent a table row made of cells
#[derive(Clone, Debug)]
pub struct Row {
    cells: Vec<Cell>,
}

impl Row {
    /// Create a new `Row` backed with `cells` vector
    pub fn new(cells: Vec<Cell>) -> Row {
        Row { cells: cells }
    }

    /// Create an row of length `size`, with empty strings stored
    pub fn empty() -> Row {
        Self::new(vec![Cell::default(); 0])
    }

    /// Get the number of cells in this row
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// Check if the row is empty (has no cell)
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// Get the height of this row
    pub fn get_height(&self) -> usize {
        let mut height = 1; // Minimum height must be 1 to print empty rows
        for cell in &self.cells {
            let h = cell.get_height();
            if h > height {
                height = h;
            }
        }
        height
    }

    /// Get the minimum width required by the cell in the column `column`.
    /// Return 0 if the cell does not exist in this row
    pub fn get_cell_width(&self, column: usize) -> usize {
        self.cells
            .get(column)
            .map(|cell| cell.get_width())
            .unwrap_or(0)
    }

    /// Get the cell at index `idx`
    pub fn get_cell(&self, idx: usize) -> Option<&Cell> {
        self.cells.get(idx)
    }

    /// Get the mutable cell at index `idx`
    pub fn get_mut_cell(&mut self, idx: usize) -> Option<&mut Cell> {
        self.cells.get_mut(idx)
    }

    /// Set the `cell` in the row at the given `column`
    pub fn set_cell(&mut self, cell: Cell, column: usize) -> Result<(), &str> {
        if column >= self.len() {
            return Err("Cannot find cell");
        }
        self.cells[column] = cell;
        Ok(())
    }

    /// Append a `cell` at the end of the row
    pub fn add_cell(&mut self, cell: Cell) {
        self.cells.push(cell);
    }

    /// Insert `cell` at position `index`. If `index` is higher than the row length,
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

    /// Returns an immutable iterator over cells
    pub fn iter(&self) -> Iter<Cell> {
        self.cells.iter()
    }

    /// Returns an mutable iterator over cells
    pub fn iter_mut(&mut self) -> IterMut<Cell> {
        self.cells.iter_mut()
    }

    /// Internal only
    fn __print<T: Write + ?Sized, F>(&self,
                                     out: &mut T,
                                     format: &TableFormat,
                                     col_width: &[usize],
                                     f: F)
                                     -> Result<(), Error>
        where F: Fn(&Cell, &mut T, usize, usize, bool) -> Result<(), Error>
    {
        for i in 0..self.get_height() {
            //TODO: Wrap this into dedicated function one day
            try!(out.write_all(&vec![b' '; format.get_indent()]));
            try!(format.print_column_separator(out, ColumnPosition::Left));
            let (lp, rp) = format.get_padding();
            for j in 0..col_width.len() {
                try!(out.write_all(&vec![b' '; lp]));
                let skip_r_fill = (j == col_width.len() - 1) &&
                                  format.get_column_separator(ColumnPosition::Right).is_none();
                match self.get_cell(j) {
                    Some(c) => try!(f(c, out, i, col_width[j], skip_r_fill)),
                    None => try!(f(&Cell::default(), out, i, col_width[j], skip_r_fill)),
                };
                try!(out.write_all(&vec![b' '; rp]));
                if j < col_width.len() - 1 {
                    try!(format.print_column_separator(out, ColumnPosition::Intern));
                }
            }
            try!(format.print_column_separator(out, ColumnPosition::Right));
            try!(out.write_all(NEWLINE));
        }
        Ok(())
    }

    /// Print the row to `out`, with `separator` as column separator, and `col_width`
    /// specifying the width of each columns
    pub fn print<T: Write + ?Sized>(&self,
                                    out: &mut T,
                                    format: &TableFormat,
                                    col_width: &[usize])
                                    -> Result<(), Error> {
        self.__print(out, format, col_width, Cell::print)
    }

    /// Print the row to terminal `out`, with `separator` as column separator, and `col_width`
    /// specifying the width of each columns. Apply style when needed
    pub fn print_term<T: Terminal + ?Sized>(&self,
                                            out: &mut T,
                                            format: &TableFormat,
                                            col_width: &[usize])
                                            -> Result<(), Error> {
        self.__print(out, format, col_width, Cell::print_term)
    }
}

impl Default for Row {
    fn default() -> Row {
        Row::empty()
    }
}

impl Index<usize> for Row {
    type Output = Cell;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.cells[idx]
    }
}

impl IndexMut<usize> for Row {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.cells[idx]
    }
}

impl<A: ToString> FromIterator<A> for Row {
    fn from_iter<T>(iterator: T) -> Row
        where T: IntoIterator<Item = A>
    {
        Self::new(iterator.into_iter().map(|ref e| Cell::from(e)).collect())
    }
}

impl<T, A> From<T> for Row
    where A: ToString,
          T: IntoIterator<Item = A>
{
    fn from(it: T) -> Row {
        Self::from_iter(it)
    }
}

impl<'a> IntoIterator for &'a Row {
    type Item = &'a Cell;
    type IntoIter = Iter<'a, Cell>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Row {
    type Item = &'a mut Cell;
    type IntoIter = IterMut<'a, Cell>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/// This macro simplifies `Row` creation
///
/// The syntax support style spec
/// # Example
/// ```
/// # #[macro_use] extern crate prettytable;
/// # fn main() {
/// // Create a normal row
/// let row1 = row!["Element 1", "Element 2", "Element 3"];
/// // Create a row with all cells formatted with red foreground color, yellow background color
/// // bold, italic, align in the center of the cell
/// let row2 = row![FrBybic => "Element 1", "Element 2", "Element 3"];
/// // Create a row with first cell in blue, second one in red, and last one with default style
/// let row3 = row![Fb->"blue", Fr->"red", "normal"];
/// // Do something with rows
/// # drop(row1);
/// # drop(row2);
/// # drop(row3);
/// # }
/// ```
///
/// For details about style specifier syntax, check doc for [`Cell::style_spec`](cell/struct.Cell.html#method.style_spec) method
#[macro_export]
macro_rules! row {
    (($($out:tt)*); $value:expr) => (vec![$($out)* cell!($value)]);
    (($($out:tt)*); $value:expr, $($n:tt)*) => (row!(($($out)* cell!($value),); $($n)*));
    (($($out:tt)*); $style:ident -> $value:expr) => (vec![$($out)* cell!($style -> $value)]);
    (($($out:tt)*); $style:ident -> $value:expr, $($n: tt)*) => (row!(($($out)* cell!($style -> $value),); $($n)*));

    ($($content:expr), *) => ($crate::row::Row::new(vec![$(cell!($content)), *]));
    ($style:ident => $($content:expr), *) => ($crate::row::Row::new(vec![$(cell!($style -> $content)), *]));
    ($($content:tt)*) => ($crate::row::Row::new(row!((); $($content)*)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_default_empty() {
        let row1 = Row::default();
        assert_eq!(row1.len(), 0);
        assert!(row1.is_empty());
    }

    #[test]
    fn get_add_set_cell() {
        let mut row = Row::from(vec!["foo", "bar", "foobar"]);
        assert_eq!(row.len(), 3);
        assert!(row.get_mut_cell(12).is_none());
        let c1 = row.get_mut_cell(0).unwrap().clone();
        assert_eq!(c1.get_content(), "foo");

        let c1 = Cell::from(&"baz");
        assert!(row.set_cell(c1.clone(), 0).is_ok());
        assert_eq!(row.get_cell(0).unwrap().get_content(), "baz");

        row.add_cell(c1.clone());
        assert_eq!(row.len(), 4);
        assert_eq!(row.get_cell(3).unwrap().get_content(), "baz");
    }

    #[test]
    fn insert_cell() {
        let mut row = Row::from(vec!["foo", "bar", "foobar"]);
        assert_eq!(row.len(), 3);
        let cell = Cell::new("baz");
        row.insert_cell(1000, cell.clone());
        assert_eq!(row.len(), 4);
        assert_eq!(row.get_cell(3).unwrap().get_content(), "baz");
        row.insert_cell(1, cell.clone());
        assert_eq!(row.len(), 5);
        assert_eq!(row.get_cell(1).unwrap().get_content(), "baz");
    }

    #[test]
    fn remove_cell() {
        let mut row = Row::from(vec!["foo", "bar", "foobar"]);
        assert_eq!(row.len(), 3);
        row.remove_cell(1000);
        assert_eq!(row.len(), 3);
        row.remove_cell(1);
        assert_eq!(row.len(), 2);
        assert_eq!(row.get_cell(0).unwrap().get_content(), "foo");
        assert_eq!(row.get_cell(1).unwrap().get_content(), "foobar");
    }
}