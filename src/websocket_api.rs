use std::error::Error;

use js_sys::Promise;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

use crate::{global_state, messagepack_helpers, requests::Request, responses::Response};

// wires up to websocket.onmessage to handle response receiving/pairing
pub fn on_websocket_message(payload: &Vec<u8>) -> Result<(), Box<dyn Error>> {
    let response = messagepack_helpers::deserialize::<Response>(&payload)?;
    log::info!("message event, received response: {response:?}");
    match &response {
        Response::Sum { request_id, body } => {
            // get global state
            let mut global_state = global_state::get_global_state()?.lock()?;
            // match
            match global_state.pending_requests.get(request_id) {
                Some(resolve) => {
                    let jsvalue_body = serde_wasm_bindgen::to_value(&body)?;
                    let _ = resolve.call1(&JsValue::NULL, &jsvalue_body); // TODO: handle error?
                }
                None => {
                    log::warn!("received non-pending request ID {request_id}");
                }
            }
            // cleanup
            global_state.pending_requests.remove(request_id);
            // call out explicit guard drop
            drop(global_state);
        }
    }
    Ok(())
}

// creates requests + sends over websocket in a fashion that allows them to be paired from websocket.onmessage handler
pub async fn call_websocket_api(
    request_id: &String,
    request: &Request,
) -> Result<JsValue, Box<dyn Error>> {
    // get state
    let mut global_state = global_state::get_global_state()?.lock()?;
    // add to pending
    log::info!("adding to pending {request_id}");
    let response_promise = Promise::new(&mut |resolve, _reject| {
        global_state
            .pending_requests
            .insert(request_id.to_string(), resolve);
    });
    // send
    log::info!("sending request {request:?}");
    let serialized_request = messagepack_helpers::serialize(&request)?;
    // send request
    global_state
        .websocket
        .send_with_u8_array(&serialized_request)
        .map_err(|_| "failed to send")?;
    // drop
    drop(global_state);
    // wait for response
    log::info!("waiting for response");
    let response = JsFuture::from(response_promise)
        .await
        .map_err(|_| "failed to resolve")?;
    log::info!("got response {response:?}");
    // return
    Ok(response)
}
