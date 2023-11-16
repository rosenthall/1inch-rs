use thiserror::Error;

// Macro to easily generate setter methods for structures.
#[macro_export]
macro_rules! builder_setter {
    ($field_name:ident, $field_type:ty) => {
        #[doc = concat!("Sets the value for the ", stringify!($field_name), " field.")]
        pub fn $field_name(mut self, value: $field_type) -> Self {
            self.$field_name = Some(value);
            self
        }
    };
}


/// Basic structure for error handling in Builder for structures where there is no need to handle other errors.
#[derive(Error, Debug, Eq, PartialEq)]
pub enum BasicBuilderError {
    /// Indicates a required field is missing its value.
    #[error("Missing {0}")]
    MissingField(&'static str),

}