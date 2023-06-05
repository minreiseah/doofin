use twsapi::core::client::EClient;
use twsapi::core::streamer::Streamer;
use twsapi::core::wrapper::Wrapper;
use std::sync::{Arc, Mutex};

pub fn sq(a: i32) -> i32 {
    a * a
}

pub struct NeoWrapper<T: Streamer + 'static> {
    pub client: Option<Arc<Mutex<EClient<NeoWrapper<T>>>>>,
    pub next_order_id: i32,
    account: String,
}

impl<T: Streamer> NeoWrapper<T> {
    pub fn new() -> Self {
        NeoWrapper {
            client: None,
            next_order_id: -1,
            account: "".toString(), }
    }

    // TODO Implement methods
}

impl<T> Wrapper for NeoWrapper<T>
where
    T: Streamer + 'static,
{
    // TODO Implement all the methods
}