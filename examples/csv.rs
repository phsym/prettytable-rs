extern crate prettytable;

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
#[cfg(feature = "csv")]
fn main() {
    use prettytable::Table;

    let table = Table::from_csv_string("ABC,DEFG,HIJKLMN\n\
                                        foobar,bar,foo\n\
                                        foobar2,bar2,foo2").unwrap();
    table.printstd();

    println!("");
    println!("{}", table.to_csv(Vec::new()).unwrap().into_string());
}

#[cfg(not(feature = "csv"))]
fn main() {}
