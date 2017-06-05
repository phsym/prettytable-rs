![License](http://img.shields.io/badge/license-BSD-lightgrey.svg)
[![Build Status](https://travis-ci.org/phsym/prettytable-rs.svg)](https://travis-ci.org/phsym/prettytable-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/wdh9klb35fed6ik9?svg=true)](https://ci.appveyor.com/project/phsym/tabprint)
[![Coverage Status](https://coveralls.io/repos/phsym/prettytable-rs/badge.svg?branch=master)](https://coveralls.io/github/phsym/prettytable-rs?branch=master)
[![Crates.io](https://img.shields.io/crates/v/prettytable-rs.svg)](https://crates.io/crates/prettytable-rs)
[![Doc.rs](https://docs.rs/prettytable-rs/badge.svg)](https://docs.rs/crate/prettytable-rs/)
[![Doc.rs](https://img.shields.io/badge/docs-master-blue.svg)](http://phsym.github.io/prettytable-rs/master)

# prettytable-rs

A formatted and aligned table printer library for [Rust](https://www.rust-lang.org).

*Copyright &copy; 2017 Pierre-Henri Symoneaux*

> THIS SOFTWARE IS DISTRIBUTED WITHOUT ANY WARRANTY <br>
> Check LICENSE.txt file for more information. <br>

# How to use

## Including

Include the library as a dependency to your project by adding the following lines to your **Cargo.toml** file:

```toml
[dependencies]
prettytable-rs = "^0.6"
```

The library requires at least `rust v1.9.0` in order to build

## Basic usage

You can start using it in the following way:

```rust
#[macro_use] extern crate prettytable;
use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;

fn main() {
    // Create the table
    let mut table = Table::new();
    // Add a row
    table.add_row(row!["ABC", "DEFG", "HIJKLMN"]);
    table.add_row(row!["foobar", "bar", "foo"]);
    // Or the more complicated way :
    table.add_row(Row::new(vec![
        Cell::new("foobar2"),
        Cell::new("bar2"),
        Cell::new("foo2")])
    );
    // Print the table to stdout
    table.printstd();
}
```

This code will produce the following output:

```text
+---------+------+---------+
| ABC     | DEFG | HIJKLMN |
+---------+------+---------+
| foobar  | bar  | foo     |
+---------+------+---------+
| foobar2 | bar2 | foo2    |
+---------+------+---------+
```

## Using macros

To make the code simpler, the `table!` macro is there for you. The following code would produce the same output :
```rust
#[macro_use] extern crate prettytable;

fn main() {
    let table = table!(["ABC", "DEFG", "HIJKLMN"],
                       ["foobar", "bar", "foo"],
                       ["foobar2", "bar2", "foo2"]
                      );
    table.printstd();
}
```

Using the `ptable!` macro would even print it on stdout for you.

Tables also support multiline cells content. As a consequence, you can print a table into another table (yo dawg ;).
For example:
```rust
let table1 = table!(["ABC", "DEFG", "HIJKLMN"],
                    ["foobar", "bar", "foo"],
                    ["foobar2", "bar2", "foo2"]
                   );
let table2 = table!(["Title 1", "Title 2"],
                    ["This is\na multiline\ncell", "foo"],
                    ["Yo dawg ;) You can even\nprint tables\ninto tables", table1]
                   );
table2.printstd();
```
Would print the following text:
```text
+-------------------------+------------------------------+
| Title 1                 | Title 2                      |
+-------------------------+------------------------------+
| This is                 | foo                          |
| a multiline             |                              |
| cell                    |                              |
+-------------------------+------------------------------+
| Yo dawg ;) You can even | +---------+------+---------+ |
| print tables            | | ABC     | DEFG | HIJKLMN | |
| into tables             | +---------+------+---------+ |
|                         | | foobar  | bar  | foo     | |
|                         | +---------+------+---------+ |
|                         | | foobar2 | bar2 | foo2    | |
|                         | +---------+------+---------+ |
+-------------------------+------------------------------+
```

Rows may have different numbers of cells. The table will automatically adapt to the largest row by printing additional empty cells in smaller rows.

## Do it with style

Tables can be added some with style like colors (background / foreground), bold, and italic, thanks to the `term` crate.

You can add `term` style attributes to cells programmatically:
```rust
extern crate term;
use term::{Attr, color};

(...)

table.add_row(Row::new(vec![
    Cell::new("foobar2")
        .with_style(Attr::ForegroundColor(color::GREEN))
        .with_style(Attr::Bold),
    Cell::new("bar2")
        .with_style(Attr::ForegroundColor(color::RED)),
    Cell::new("foo2")])
);
```

Or you can use the style string:
```rust
Cell::new("foo2").style_spec("FrByc")
```

Where **FrBybc** means **F**oreground: **r**ed, **B**ackground: **y**ellow, **b**old, **c**enter.

With macros it's even simpler:

In rows, for each cells:
```rust
row![FrByb->"ABC", FrByb->"DEFG", "HIJKLMN"];
```
Or for the whole row:
```rust
row![FY => "styled", "bar", "foo"];
```
In tables, for each cells:
```rust
table!([FrBybl->"A", FrBybc->"B", FrBybr->"C"], [123, 234, 345, 456]);
```
Or for each rows:
```rust
table!([Frb => "A", "B", "C"], [Frb => 1, 2, 3, 4], [1, 2, 3]);
```
Or a mix:
```rust
table!([Frb => "A", "B", "C"], [Frb->1, Fgi->2, 3, 4], [1, 2, 3]);
```

### List of style specifiers:

* **F** : **F**oreground (must be followed by a color specifier)
* **B** : **B**ackground (must be followed by a color specifier)
* **b** : **b**old
* **i** : **i**talic
* **u** : **u**nderline
* **c** : Align **c**enter
* **l** : Align **l**eft
* **r** : Align **r**ight
* **d** : **d**efault style

### List of color specifiers:

* **r** : Red
* **b** : Blue
* **g** : Green
* **y** : Yellow
* **c** : Cyan
* **m** : Magenta
* **w** : White
* **d** : Black

Capital letters are for **bright** colors. Eg:
* **R** : Bright Red
* **B** : Bright Blue
* ... and so on ...

## Slicing

Tables can be sliced into immutable borrowed subtables.
Slices are of type `prettytable::TableSlice<'a>`.

For example:
```rust
use prettytable::Slice;
(...)
let slice = table.slice(2..5);
table.printstd();
```
Would print a table with only lines 2, 3 and 4 from `table`.

Other `Range` syntax are supported. For example:
```rust
table.slice(..); // Returns a borrowed immutable table with all rows
table.slice(2..); // Returns a table with rows starting at index 2
table.slice(..3); // Returns a table with rows until the one at index 3
```

## Customize your table look and feel

You can customize the look and feel of a table by providing it a `prettytable::format::TableFormat`.
For example you can change the characters used for borders, junctions, column separations or line separations.
To proceed, you can create a new `TableFormat` object and call the setter methods to configure it,
or you can use the more convenient `prettytable::format::FormatBuilder` structure.

For example:
```rust
let mut table = /* Initialize table */;
let format = format::FormatBuilder::new()
    .column_separator('|')
    .borders('|')
    .separators(
        &[format::LinePosition::Top, format::LinePosition::Bottom],
        format::LineSeparator::new('-', '+', '+', '+')
    )
    .padding(1, 1)
    .build();
table.set_format(format);
```
Would give a table like the following:
```
+-------------+------------+
| Title 1     | Title 2    |
| Value 1     | Value 2    |
| Value three | Value four |
+-------------+------------+
```

For convenience, some predefined formats are provided in the module `prettytable::format::consts`.
For example:
```rust
table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
```
Would give a table like the following:
```
+-------------+------------+
| Title 1     | Title 2    |
+-------------+------------+
| Value 1     | Value 2    |
| Value three | Value four |
+-------------+------------+
```
or
```rust
table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
```
Would give:
```
Title 1     | Title 2
------------+------------
Value 1     | Value 2
Value three | Value four
```

Check API documentation for the full list of available predefined formats.

## CSV import/export
Tables can be imported from and exported to **CSV**.  This is possible thanks to the default & optional feature `csv`.
> The `csv` feature may become deactivated by default on future major releases.

### Importing
A `Table` can be imported directly from a string
```rust
Table::from_csv_string("ABC,DEFG,HIJKLMN\n\
                        foobar,bar,foo\n\
                        foobar2,bar2,foo2");
```
Or from CSV files with 
```rust
Table::from_csv_file<P: AsRef<Path>>(csv_p: P) -> csv::Result<Table>
```
> Those 2 ways of importing CSV assumes a CSV format with `no headers`, and delimitred with `comas`

Import can also be performed from a csv reader which allows for more customization around the CSV format.
```rust
Table::from_csv<R: Read>(reader: &mut csv::Reader<R>) -> Table
```

### Exporting
The same way as used when importing CSV, exporting can be performed using the 2 following functions
```rust
Table::to_csv<W: Write>(&self, w: W) -> csv::Result<csv::Writer<W>>
to_csv_writer<W: Write>(&self,
                        mut writer: csv::Writer<W>)
                        -> csv::Result<csv::Writer<W>>
```


## Note on line endings
By default, the library prints tables with platform specific line ending. Thin means on Windows,
newlines will be rendered with `\r\n` while on other platforms they will be rendered with `\n`.
Since `v0.6.3`, platform specific line endings are activated though the default feature `win_crlf`, which can be deactivated.
When this feature is deactivated (for instance with the `--no-default-features` flag in cargo), line endings will be rendered with `\n`
on any platform.

This customization capability will probably move to Formatting API in `v0.7`.

Additional examples are provided in the documentation and in [examples](./examples/) directory.
