
// The function inserts optional value to params, but only if value is not 'None'
pub(crate) fn insert_optional_param<'a>(params: &mut Vec<(&'a str, String)>, param_name: &'a str, value: Option<String>) {
    if let Some(v) = value {
        params.push((param_name, v));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_optional_param_with_both_cases() {
        let mut params = vec![("existing_param", "existing_value".to_string())];

        // Test that no field is added when the value is None
        insert_optional_param(&mut params, "none_param", None);
        assert_eq!(params.len(), 1); // No field should be added

        // Test that a field is added when the value is Some
        insert_optional_param(&mut params, "new_param", Some("new_value".to_string()));
        assert_eq!(params.len(), 2); // A new field should be added
        assert_eq!(params[1], ("new_param", "new_value".to_string())); // Check the added field
    }
}
