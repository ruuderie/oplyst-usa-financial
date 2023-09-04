pub fn remove_quotes(s: &polars::prelude::AnyValue<'_>) -> String {
    let s = s.to_string();
    s.trim_matches('\"').to_string()
}
pub fn remove_quotes_from_string(s: &String) -> String {
    s.trim_matches('\"').to_string()
}
pub fn remove_quotes_from_str(s: &str) -> String {
    s.trim_matches('\"').to_string()
}