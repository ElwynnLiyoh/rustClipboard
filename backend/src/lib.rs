pub mod model;
pub mod redis;
pub mod service;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::{model::ClipboardRequest, redis::rudis, service};

    #[test]
    fn test_serivce() {
        let req = ClipboardRequest::new("rust clipboard", 5, 1);
        let hash_code = service::save_text(&req).unwrap();
        assert!(rudis::key_exists(hash_code.as_str()).unwrap());
        assert!(service::get_text(&hash_code).is_ok());
        assert!(!rudis::key_exists(hash_code.as_str()).unwrap());
    }
}
