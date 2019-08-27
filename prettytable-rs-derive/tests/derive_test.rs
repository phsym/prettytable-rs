use prettytable_derive::TableElem;
use prettytable::TableElem;

#[derive(TableElem)]
struct NameStruct {
	name: String,
	surname: String,
}

#[test]
fn test_get_field_name() {
    assert_eq!(vec!["name", "surname"], NameStruct::get_field_name());
}

#[test]
fn test_get_field() {
    let t = NameStruct {
        name: "real_name".to_string(),
        surname: "real_surname".to_string(),
    };

    assert_eq!(vec!["real_name", "real_surname"], t.get_field());
}