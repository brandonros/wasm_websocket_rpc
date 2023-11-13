use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Request {
    Sum {
        request_id: String,
        body: SumRequest,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SumRequest {
    pub operands: Vec<usize>,
}
