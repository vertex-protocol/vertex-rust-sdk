use crate::serialize_utils::{WrappedBytes32, WrappedU32, WrappedU64};

pub fn wrapped_option_vec_u32(optional_u32_vec: Option<Vec<u32>>) -> Option<Vec<WrappedU32>> {
    optional_u32_vec
        .as_ref()
        .map(|ids| WrappedU32::wrap_vec_u32(&ids))
}

pub fn wrapped_bytes_32(optional_bytes: Option<[u8; 32]>) -> Option<WrappedBytes32> {
    match optional_bytes {
        Some(unwrapped) => Some(WrappedBytes32(unwrapped)),
        None => None,
    }
}

pub fn wrapped_option_u64(optional_u64: Option<u64>) -> Option<WrappedU64> {
    match optional_u64 {
        Some(unwrapped) => Some(WrappedU64(unwrapped)),
        None => None,
    }
}

pub fn wrapped_option_bytes32(optional_bytes: Option<[u8; 32]>) -> Option<WrappedBytes32> {
    match optional_bytes {
        Some(unwrapped) => Some(WrappedBytes32(unwrapped)),
        None => None,
    }
}

pub fn wrapped_option_vec_bytes32(
    optional_bytes_vec: Option<Vec<[u8; 32]>>,
) -> Option<Vec<WrappedBytes32>> {
    match optional_bytes_vec {
        None => None,
        Some(unwrapped_bytes_vec) => Some(
            unwrapped_bytes_vec
                .iter()
                .map(|&bytes| WrappedBytes32(bytes))
                .collect(),
        ),
    }
}

pub fn optional_bool_to_string(optional_bool: Option<bool>) -> Option<String> {
    match optional_bool {
        None => None,
        Some(unwrapped_bool) => Some(unwrapped_bool.to_string()),
    }
}
