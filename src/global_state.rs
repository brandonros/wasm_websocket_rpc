use std::{
    collections::HashMap,
    error::Error,
    sync::{Mutex, OnceLock},
};

use js_sys::Function;
use web_sys::WebSocket;

pub struct GlobalState {
    pub websocket: WebSocket,
    pub pending_requests: HashMap<String, Function>,
}

// TODO: is this ok?
unsafe impl Send for GlobalState {}
unsafe impl Sync for GlobalState {}

static GLOBAL_STATE: OnceLock<Mutex<GlobalState>> = OnceLock::new();

pub fn get_global_state() -> Result<&'static Mutex<GlobalState>, Box<dyn Error>> {
    GLOBAL_STATE.get().ok_or("GLOBAL_STATE not set".into())
}

pub fn init(websocket: WebSocket) {
    let _ = GLOBAL_STATE.set(Mutex::new(GlobalState {
        websocket,
        pending_requests: HashMap::new(),
    }));
}
