pub mod execute;
pub mod indexer;
pub mod query;
pub mod trigger;
pub mod utils;

#[doc(hidden)]
#[macro_export]
macro_rules! vertex_builder {
    ($name:ident, $trait_bounds:ident, $($field:ident: $ftype:ty),*; $($custom_impl:tt)*) => {
        #[derive(Clone)]
        pub struct $name<'a, V: $trait_bounds> {
            vertex: &'a V,
            $( $field: Option<$ftype>, )*
        }

        impl<'a, V: $trait_bounds> $name<'a, V> {
            pub fn new(vertex: &'a V) -> Self {
                Self {
                    vertex,
                    $( $field: None, )*
                }
            }

            $(
            pub fn $field(&self, $field: $ftype) -> Self {
                Self {
                    $field: Some($field),
                    ..self.clone()
                }
            }
            )*

            $($custom_impl)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! build_and_call {
    ($instance:ident, $method_name:ident, $call:ident => $output_type:ty, async_build) => {
        pub async fn $method_name(&$instance) -> Result<$output_type> {
            $instance.vertex.$call($instance.build().await?).await
        }
    };
    ($instance:ident, $method_name:ident, $execute_call:ident => $output_type:ty) => {
        pub async fn $method_name(&$instance) -> Result<$output_type> {
            $instance.vertex.$execute_call($instance.build()?).await
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! fields_to_vars {
    ($instance:ident, $($tt:tt),+ $(,)? ) => {
        $(
            fields_to_vars!(@single $instance, $tt);
        )+
    };
    (@single $instance:ident, ($field:ident : $transform:ident)) => {
        let $field = $instance.$field.$transform().ok_or(none_error(stringify!($field)))?;
    };
    (@single $instance:ident, $field:ident) => {
        let $field = $instance.$field.ok_or(none_error(stringify!($field)))?;
    };
}
