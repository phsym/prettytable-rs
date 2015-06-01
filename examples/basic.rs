#[macro_use] extern crate tabprint;
use tabprint::Table;
use tabprint::row::Row;
use tabprint::cell::Cell;

/*
	Following main function will print :
	+---------+------+---------+
	| ABC     | DEFG | HIJKLMN |
	+---------+------+---------+
	| foobar  | bar  | foo     |
	+---------+------+---------+
	| foobar2 | bar2 | foo2    |
	+---------+------+---------+
	Modified : 
	+---------+------+---------+
	| ABC     | DEFG | HIJKLMN |
	+---------+------+---------+
	| foobar  | bar  | foo     |
	+---------+------+---------+
	| foobar2 | bar2 | new_foo |
	+---------+------+---------+
*/
fn main() {
    let mut table = Table::new(row!["ABC", "DEFG", "HIJKLMN"]);
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
    
    // The same table can be built the following way :
    let _table = table!(["ABC", "DEFG", "HIJKLMN"],
    				   ["foobar", "bar", "foo"],
    				   ["foobar2", "bar2", "foo2"]
    				  );
}
