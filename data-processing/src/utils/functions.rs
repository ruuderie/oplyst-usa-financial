use polars::datatypes::AnyValue;

pub fn remove_quotes_from_anyvalue(s: &polars::prelude::AnyValue<'_>) -> String {
    let s = s.to_string();
    s.trim_matches('\"').to_string()
}

pub fn remove_quotes_from_str(s: &str) -> String {
    s.trim_matches('\"').to_string()
}

pub fn any_value_to_sea_orm_value(val: &AnyValue) -> String {
    // Check the type of the `AnyValue` and convert it to the appropriate SeaORM value type.
    if let Some(v) = val.get_str() {
        v.to_string()
    } else {
        // Add other type checks and conversions as necessary
        println!("{:?} <--", val);
        // This is a fallback; you might want to handle this case more gracefully
        panic!("Unsupported type!")
    }
}
