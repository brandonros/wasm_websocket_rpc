use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Response {
    Sum {
        request_id: String,
        body: SumResponse,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SumResponse {
    sum: usize,
}
