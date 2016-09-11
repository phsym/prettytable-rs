extern crate prettytable;
use prettytable::Table;

/*
	Following main function will print :
	+---------+------+---------+
	| ABC     | DEFG | HIJKLMN |
	+---------+------+---------+
	| foobar  | bar  | foo     |
	+---------+------+---------+
	| foobar2 | bar2 | foo2    |
	+---------+------+---------+

    ABC,DEFG,HIJKLMN
    foobar,bar,foo
    foobar2,bar2,foo2
*/
fn main() {
    let table = Table::from_csv_string("ABC,DEFG,HIJKLMN\n\
                                        foobar,bar,foo\n\
                                        foobar2,bar2,foo2").unwrap();
    table.printstd();

    println!("");
    println!("{}", table.to_csv(Vec::new()).unwrap().into_string());
}
