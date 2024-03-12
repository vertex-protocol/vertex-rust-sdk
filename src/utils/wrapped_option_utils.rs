use crate::serialize_utils::{WrappedBytes32, WrappedU32, WrappedU64};

pub fn wrapped_option_vec_u32(optional_u32_vec: Option<Vec<u32>>) -> Option<Vec<WrappedU32>> {
    optional_u32_vec
        .as_ref()
        .map(|ids| WrappedU32::wrap_vec_u32(ids))
}

pub fn wrapped_bytes_32(optional_bytes: Option<[u8; 32]>) -> Option<WrappedBytes32> {
    optional_bytes.map(WrappedBytes32)
}

pub fn wrapped_option_u64(optional_u64: Option<u64>) -> Option<WrappedU64> {
    optional_u64.map(WrappedU64)
}

pub fn wrapped_option_bytes32(optional_bytes: Option<[u8; 32]>) -> Option<WrappedBytes32> {
    optional_bytes.map(WrappedBytes32)
}

pub fn wrapped_option_vec_bytes32(
    optional_bytes_vec: Option<Vec<[u8; 32]>>,
) -> Option<Vec<WrappedBytes32>> {
    optional_bytes_vec.map(|unwrapped_bytes_vec| {
        unwrapped_bytes_vec
            .iter()
            .map(|&bytes| WrappedBytes32(bytes))
            .collect()
    })
}

pub fn optional_bool_to_string(optional_bool: Option<bool>) -> Option<String> {
    optional_bool.map(|unwrapped_bool| unwrapped_bool.to_string())
}
