#![no_main]
use libfuzzer_sys::fuzz_target;

use prettytable::{Table, Row, Cell};

use prettytable::format::Alignment::{self, *};

fuzz_target!(|data: Vec<Vec<(String, u8)>>| {
    fn align_from_u8(x: u8) -> Alignment {
        match x {
            0 => LEFT,
            1 => CENTER,
            _ => RIGHT,
        }
    }
    let mut pt = prettytable::Table::new();
    for row in data {
        let cells = row.into_iter().map(|x| Cell::new_align(&x.0, align_from_u8(x.1))).collect();
        pt.add_row(Row::new(cells));
    }

    let _ = pt.print(&mut std::io::sink());
});
