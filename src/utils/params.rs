
// The function inserts optional value to params, but only if value is not 'None'
pub(crate) fn insert_optional_param<'a>(params: &mut Vec<(&'a str, String)>, param_name: &'a str, value: Option<String>) {
    if let Some(v) = value {
        params.push((param_name, v));
    }
}
