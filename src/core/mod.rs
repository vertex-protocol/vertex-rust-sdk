pub mod base;
pub mod builder;
pub mod execute;
pub mod indexer;
pub mod query;

pub use base::VertexBase;
pub use builder::VertexBuilder;
pub use execute::VertexExecute;
pub use indexer::VertexIndexer;
pub use query::VertexQuery;

#[doc(hidden)]
#[macro_export]
macro_rules! map_response {
    ($response:expr, $expected_type:path) => {
        match $response {
            $expected_type(data) => Ok(data),
            _ => Err(eyre!(concat!(
                "expected ",
                stringify!($expected_type),
                " response"
            ))),
        }
    };
}
