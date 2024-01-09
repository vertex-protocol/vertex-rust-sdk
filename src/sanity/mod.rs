pub mod execute;
pub mod indexer;
pub mod query;

#[doc(hidden)]
#[macro_export]
macro_rules! print_json {
    ($data:expr) => {{
        let label = stringify!($data);
        println!("{}: {}", label, serde_json::to_string_pretty(&$data)?);
    }};
}
