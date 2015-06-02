![License](http://img.shields.io/badge/license-BSD-lightgrey.svg)
[![Build Status](https://travis-ci.org/phsym/tabprint.svg)](https://travis-ci.org/phsym/tabprint)
[![Build status](https://ci.appveyor.com/api/projects/status/wdh9klb35fed6ik9?svg=true)](https://ci.appveyor.com/project/phsym/tabprint)

# tabprint

*Copyright &copy; 2015 Pierre-Henri Symoneaux*

> THIS SOFTWARE IS DISTRIBUTED WITHOUT ANY WARRANTY <br>
> Check LICENSE.txt file for more information. <br>

A formatted and aligned table printer written in rust. **This is a work in progress for now**.

# How to build

As usual with Cargo project, simply run

> cargo build

And to build html documentation, run

> cargo doc

# How to use
More often, you will include the library as a dependency to your project. In order to do this, add the following lines to your **Cargo.toml** file :

```toml
[dependencies.tabprint]
git = "https://github.com/phsym/tabprint.git"

```

Then you can start using it the following way :

```rust
extern crate tabprint;
use tabprint::Table;
use tabprint::row::Row;
use tabprint::cell::Cell;

fn main() {
	// Create the table
	let mut table = Table::new();
	// Add a row
	table.add_row(row!["ABC", "DEFG", "HIJKLMN"]);
    table.add_row(row!["foobar", "bar", "foo"]);
    // Or the more complicated way :
    table.add_row(Row::new(vec![
    		Cell::new(&"foobar2".to_string()),
    		Cell::new(&"bar2".to_string()),
    		Cell::new(&"foo2".to_string())])
    	);
    table.printstd();
}
```

This code will produce the following output :

```text
+---------+------+---------+
| ABC     | DEFG | HIJKLMN |
+---------+------+---------+
| foobar  | bar  | foo     |
+---------+------+---------+
| foobar2 | bar2 | foo2    |
+---------+------+---------+
```

To make the code simpler, the `table!` macro is there for you. The following code would produce the exact same output :
```rust
#[macro_use] extern crate tabprint;

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
For example, the following code
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
Would print the following text :
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

Additional examples are provided in documentation and in [examples](./examples/) directory
