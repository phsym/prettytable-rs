#![no_main]
use libfuzzer_sys::fuzz_target;

use prettytable::{Table, Row, Cell};

fuzz_target!(|data: Vec<Vec<String>>| {
    let mut pt = prettytable::Table::new();
    for row in data {
        let cells = row.into_iter().map(|x| Cell::new(&x)).collect();
        pt.add_row(Row::new(cells));
    }

    let _ = pt.print(&mut std::io::sink());
});
