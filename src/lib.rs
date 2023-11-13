#![allow(non_snake_case)]

mod global_state;
mod messagepack_helpers;
mod requests;
mod responses;
mod websocket_api;

use js_sys::{ArrayBuffer, Promise, Uint8Array};
use log::Level;
use requests::{Request, SumRequest};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{BinaryType, ErrorEvent, MessageEvent, WebSocket};

#[wasm_bindgen]
pub fn init_logging() -> Result<(), JsValue> {
    // init logging
    console_log::init_with_level(Level::Debug)
        .map_err(|_| JsValue::from_str("Failed to init logging"))?;
    // return
    Ok(())
}

#[wasm_bindgen]
pub async fn init_websocket() -> Result<JsValue, JsValue> {
    // Connect to an echo server
    let websocket = WebSocket::new("ws://127.0.0.1:3000/")?;
    // For small binary messages, like CBOR, Arraybuffer is more efficient than Blob handling
    websocket.set_binary_type(BinaryType::Arraybuffer);
    // onmessage callback
    let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        log::info!("onmessage event");
        if let Ok(abuf) = e.data().dyn_into::<ArrayBuffer>() {
            let payload = Uint8Array::new(&abuf).to_vec();
            match websocket_api::on_websocket_message(&payload) {
                Ok(_) => {}
                Err(err) => log::error!("error processing websocket message {err:?}"),
            }
        } else {
            log::warn!("message event, received Unknown: {:?}", e.data());
        }
    });
    websocket.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();
    // close callback
    let onclose_callback = Closure::<dyn FnMut()>::new(move || {
        log::warn!("socket closed");
    });
    websocket.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
    onclose_callback.forget();
    // open callback
    let onopen_promise = Promise::new(&mut |resolve, reject| {
        let onopen_callback = Closure::<dyn FnMut()>::new(move || {
            log::info!("socket opened");
            let _ = resolve.call0(&JsValue::NULL); // TODO: handle error?
        });
        websocket.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
        // error callback
        let onerror_callback = Closure::<dyn FnMut(_)>::new(move |err: ErrorEvent| {
            log::error!("error event: {:?}", err);
            let _ = reject.call1(&JsValue::NULL, &err); // TODO: handle error?
        });
        websocket.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();
    });
    let onopen_future = JsFuture::from(onopen_promise);
    onopen_future.await?;
    // set static
    global_state::init(websocket);
    // return
    Ok(JsValue::TRUE)
}

#[wasm_bindgen]
pub async fn sum(operands: &[usize]) -> Result<JsValue, JsValue> {
    // TODO: operands paramater
    // build request
    let request_id = Uuid::new_v4();
    let request = Request::Sum {
        request_id: request_id.to_string(),
        body: SumRequest {
            operands: operands.to_vec(),
        },
    };
    // call api
    match websocket_api::call_websocket_api(&request_id.to_string(), &request).await {
        Ok(response) => Ok(response),
        Err(err) => {
            log::error!("{err:?}");
            let rejected_promise = Promise::reject(&format!("{err:?}").into());
            Ok(rejected_promise.into())
        }
    }
}
