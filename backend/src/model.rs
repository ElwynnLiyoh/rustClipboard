use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ClipboardRequest<'r> {
    pub text: &'r str,
    pub expire_time: usize, // seconds
    pub access_limit: u8,
}

impl<'r> ClipboardRequest<'r> {
    pub fn new(text: &'r str, expire_time: usize, access_limit: u8) -> Self {
        ClipboardRequest {
            text,
            expire_time,
            access_limit,
        }
    }

    /// convert minutes to seconds
    pub fn min2sec(&mut self) {
        self.expire_time *= 60
    }
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ClipboardResponse {
    code: i16,
    msg: String,
    data: String,
}

impl ClipboardResponse {
    pub fn new(code: i16, msg: String, data: String) -> Self {
        ClipboardResponse { code, msg, data }
    }

    pub fn success(data: String) -> Self {
        ClipboardResponse {
            code: 0,
            msg: String::from("success"),
            data,
        }
    }

    pub fn failed(msg: String) -> Self {
        Self::new(-1, msg, String::new())
    }
}
