#[macro_use] extern crate tabprint;
use tabprint::Table;
use tabprint::row::Row;
use tabprint::cell::Cell;

#[allow(dead_code)]
fn main() {
    let mut table = Table::new(row!["ABC", "DEFG", "HIJKLMN"]);
    table.add_row(row!["foobar", "bar", "foo"]).unwrap();
    table.add_row(Row::new(vec![
    		Cell::new(&"foobar2".to_string()),
    		Cell::new(&"bar2".to_string()),
    		Cell::new(&"foo2".to_string())])
    	).unwrap();
    table.printstd();
    println!("Modified : ");
    table.separators('*', '*', '*');
    table.set_element(&"new_foo".to_string(), 2, 1).unwrap();
    table.printstd();
    
    let table = table!(["A", "B", "C"], ["A\nBCCZZZ\nDDD", 2, table]);
    table.printstd();
    println!("{:?}", table);
}
