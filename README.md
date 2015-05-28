![License](http://img.shields.io/badge/license-BSD-lightgrey.svg)
[![Build Status](https://travis-ci.org/phsym/tabprint.svg)](https://travis-ci.org/phsymtabprint)

# tabprint

*Copyright &copy; 2015 Pierre-Henri Symoneaux*

> THIS SOFTWARE IS DISTRIBUTED WITHOUT ANY WARRANTY <br>
> Check LICENSE.txt file for more information. <br>

A formatted and aligned table printer written in rust

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
#[macro_use] extern crate tabprint;
use tabprint::Table;

fn main() {
	let mut table = Table::new(vec!["ABC".to_string(), "DEFG".to_string(), "HIJKLMN".to_string()]);
    table.add_row(vec!["foobar".to_string(), "bar".to_string(), "foo".to_string()]).unwrap();
    table.add_row(vec!["foobar2".to_string(), "bar2".to_string(), "foo2".to_string()]).unwrap();
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

Additional examples are provided in documentation and in [examples](./examples/) directory
