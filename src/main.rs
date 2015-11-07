#[macro_use] extern crate prettytable;
use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;
use prettytable::format::*;

extern crate term;
use term::{Attr, color};

#[allow(dead_code)]
fn main() {
    let mut table = Table::new();
    table.add_row(row!["ABC", "DEFG", "HIJKLMN"]);
    table.add_row(row!["foobar", "bar", "foo"]);
    table.add_row(Row::new(vec![
    		Cell::new("foobar2"),
			Cell::new("bar2").with_style(Attr::ForegroundColor(color::RED)),
    		Cell::new("foo2")])
    	);
    for cell in table.column_iter_mut(2) {
    	cell.align(Align::RIGHT);
    }
    for cell in table.column_iter_mut(1) {
    	cell.align(Align::CENTER);
    }
    table.printstd();
    println!("Modified : ");
    table.set_element("new_foo", 2, 1).unwrap();
    table.printstd();
    
 	ptable!(["A", "B", "C"], [1, 2, 3, 4]);
 	   
    let mut table = table!(["A", "B", "C"], [1, 2, 3, 4], ["A\nBCCZZZ\nDDD", 2, table]);
    table.set_titles(row!["Title 1", "Title 2"]);
    table.set_format(FORMAT_DEFAULT);
    table.printstd();
//    println!("{:#?}", table);
}
