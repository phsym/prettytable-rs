//! CSV impl and reexported types

extern crate csv;

pub use self::csv::{Reader, Writer, Result, ReaderBuilder};
use std::path::Path;
use std::io::{Read, Write};

impl<'a> super::TableSlice<'a> {
    /// Write the table to the specified writer.
    pub fn to_csv<W: Write>(&self, w: W) -> Result<Writer<W>> {
        self.to_csv_writer(Writer::from_writer(w))
    }

    /// Write the table to the specified writer.
    ///
    /// This allows for format customisation.
    pub fn to_csv_writer<W: Write>(&self,
                                mut writer: Writer<W>)
                                -> Result<Writer<W>> {
        for title in self.titles {
            writer.write_record(title.iter().map(|c| c.get_content()))?;
        }
        for row in self.rows {
            writer.write_record(row.iter().map(|c| c.get_content()))?;
        }

        writer.flush()?;
        Ok(writer)
    }
}

impl super::Table {
    /// Create a table from a CSV string
    ///
    /// For more customisability use `from_csv()`
    pub fn from_csv_string(csv_s: &str) -> Result<Self> {
        Ok(Self::from_csv(
            &mut ReaderBuilder::new()
                .has_headers(false)
                .from_reader(csv_s.as_bytes())))
    }

    /// Create a table from a CSV file
    ///
    /// For more customisability use `from_csv()`
    pub fn from_csv_file<P: AsRef<Path>>(csv_p: P) -> Result<Self> {
        Ok(Self::from_csv(
            &mut ReaderBuilder::new()
                .has_headers(false)
                .from_path(csv_p)?))
    }

    /// Create a table from a CSV reader
    pub fn from_csv<R: Read>(reader: &mut Reader<R>) -> Self {
        Self::init(reader
                        .records()
                        .map(|row| {
                                super::row::Row::new(row.unwrap()
                                            .into_iter()
                                            .map(|cell| super::cell::Cell::new(&cell))
                                            .collect())
                            })
                        .collect())
    }

    
    /// Write the table to the specified writer.
    pub fn to_csv<W: Write>(&self, w: W) -> Result<Writer<W>> {
        self.as_ref().to_csv(w)
    }

    /// Write the table to the specified writer.
    ///
    /// This allows for format customisation.
    pub fn to_csv_writer<W: Write>(&self, writer: Writer<W>) -> Result<Writer<W>> {
        self.as_ref().to_csv_writer(writer)
    }
}