#[macro_use] extern crate prettytable;
use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;
use prettytable::format::*;

#[allow(dead_code)]
fn main() {
    let mut table = Table::new();
    table.add_row(row!["ABC", "DEFG", "HIJKLMN"]);
    table.add_row(row!["foobar", "bar", "foo"]);
    table.add_row(Row::new(vec![
    		Cell::new(&"foobar2".to_string()),
    		Cell::new(&"bar2".to_string()),
    		Cell::new(&"foo2".to_string())])
    	);
    table.printstd();
    println!("Modified : ");
    table.set_element(&"new_foo".to_string(), 2, 1).unwrap();
    table.printstd();
    
    let mut table = table!(["A", "B", "C"], [1, 2, 3, 4], ["A\nBCCZZZ\nDDD", 2, table]);
    table.set_titles(row!["Title 1", "Title 2"]);
    table.set_format(FORMAT_DEFAULT);
    table.printstd();
    println!("{:?}", table);
}
