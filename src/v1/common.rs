use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

#[macro_export]
macro_rules! impl_builder_methods {
    ($builder:ident, $($field:ident: $field_type:ty),*) => {
        $(
            impl $builder {
                pub fn $field(mut self, $field: $field_type) -> Self {
                    self.$field = Some($field);
                    self
                }
            }
        )*
    };
}
