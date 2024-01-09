use crate::utils::time::timestamp;
use rand::Rng;

const DEFAULT_RECV_SECS: u64 = 90;

pub fn order_nonce(recv_time: Option<u64>) -> u64 {
    let recv_time = recv_time.unwrap_or(default_recv_time());
    let mut rng = rand::thread_rng();
    let random_int = rng.gen_range(0..1000);
    let nonce = (recv_time << 20) + random_int;
    nonce
}

pub fn default_recv_time() -> u64 {
    (timestamp() + DEFAULT_RECV_SECS) * 1000
}
