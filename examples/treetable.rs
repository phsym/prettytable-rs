use prettytable::tree;
use prettytable::{format, row, Table};

/// Following main function will print :
/// ```txt
///  t1        t2
///   1         a
///   ├─ 2      b
///   │  └─ 3   b
///   └─ 4      c
///   5         z
///   └─ 6      z
/// ```
fn main() {
    // Create data
    let data = vec![
        ("1", 1, "a"),
        ("1/2", 2, "b"),
        ("1/2/3", 3, "b"),
        ("1/4", 4, "c"),
        ("5", 5, "z"),
        ("5/6", 6, "z"),
    ];
    // Create the table
    let mut table = Table::new();
    let format = format::FormatBuilder::new()
        .separators(&[], format::LineSeparator::new('-', '+', '+', '+'))
        .padding(1, 1)
        .build();
    table.set_format(format);
    table.set_titles(row![bl->"t1", br->"t2"]);
    let prefixes = tree::provide_prefix(&data, |parent, item| {
        parent.0.split("/").count() + 1 == item.0.split("/").count() && item.0.starts_with(parent.0)
    });
    for (datum, prefix) in data.iter().zip(prefixes.iter()) {
        table.add_row(row![
            &format!("{} {}", prefix, datum.1),
            r-> &format!("{}", datum.2),
        ]);
    }

    // Print the table to stdout
    table.printstd();
}
