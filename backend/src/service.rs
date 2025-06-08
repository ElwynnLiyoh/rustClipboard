use crate::redis::*;
use log::error;

/// gets the text associated with the provided code from Redis
/// # Arguments
/// * `code` - The code to retrieve the text for
/// # Returns
/// * `Result<String, String>` - The text associated with the code, 
/// or an error message if there was an error
pub fn get_text(code: &str) -> Result<String, String> {
    match rudis::get_text(code) {
        Ok(text_opt) => match text_opt {
            Some(text) => return Ok(text),
            None => return Err("Code not found".to_string()),
        },
        Err(e) => {
            error!("get_text error: {}, code: {}", e, code);
            return Err("Failed to retrieve text".to_string());
        }
    };
}

/// saves the provided text to Redis
/// returns the code for the saved text
///
/// # Arguments
/// * `req` - A reference to a `ClipboardRequest` containing the text,
/// # Ruturns
/// `Result<String, String>` - The code for the saved text, 
/// or an error message if there was an error
use crate::model::ClipboardRequest;
use crate::utils::{conf, tools};
pub fn save_text(req: &ClipboardRequest) -> Result<String, String> {
    // check if the key is expired
    let max_expire_time = conf::read_custom_config()
        .get_int("general.max_expire_time")
        .unwrap_or(180) as usize;
    if req.expire_time > max_expire_time {
        return Err("Expire time exceeds the maximum allowed value".to_string());
    }

    // check if content exceeds the maximum allowed value
    let max_content_limit = conf::read_custom_config()
        .get_int("general.max_content_limit")
        .unwrap_or(1024_000) as usize;
    if req.text.len() > max_content_limit {
        return Err("Content exceeds the maximum allowed value".to_string());
    }

    let hash_code = tools::calc_hash(req.text);
    match rudis::set_text(&hash_code, &req.text, req.expire_time, req.access_limit) {
        Ok(_) => Ok(hash_code), // Return the hash code
        Err(e) => {
            error!("save_text error: {}, request: {:?}", e, req);
            return Err("Cannot save the test due to unknown error".to_string());
        }
    }
}
