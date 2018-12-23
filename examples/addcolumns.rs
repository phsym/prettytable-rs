#[macro_use]
extern crate prettytable;
use prettytable::{Table, Row, Cell};

fn main() {
    let mut table = Table::new();
    table.add_column(vec!["foobar","foobar2"]);
    table.add_column(vec!["bar","bar2","bar3","bar4","bar5"]);
    table.add_column(vec!["foo","foo2","foo3"]);
    table.set_titles(row!["C1","C2","C3"]);
    table.printstd();
}
