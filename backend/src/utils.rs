pub mod conf {
    use config::{Config, File};
    fn read_config_file(file_name: &str) -> Config {
        let settings = Config::builder()
            .add_source(File::with_name(file_name))
            .build()
            .unwrap();
        settings
    }

    pub fn read_custom_config() -> Config {
        read_config_file("Config.toml")
    }

    pub fn read_table<T: for<'de> serde::Deserialize<'de>>(table_name: &str) -> T {
        let settings = read_custom_config();
        settings.get::<T>(table_name).unwrap()
    }
}

pub mod tools {
    use super::conf::*;
    use sha2::{Digest, Sha256};

    /// calculate hash
    /// 
    /// # Arguments
    /// * `text` - the text to be hashed
    /// # Returns
    /// * `String` - the hash of the text, length is determined by the config
    /// 
    /// this example checks both reading the config and calculating the hash
    /// # Example
    /// ```rust
    /// use clipboard::utils::{conf, tools};
    /// let text = "rust clipboard";
    /// let hash = tools::calc_hash(text);
    /// let len: u8 = conf::read_custom_config()
    ///     .get_int("general.hash_len")
    ///     .unwrap_or(4) as u8;
    ///     assert_eq!(
    ///        hash,
    ///        "eabdf7875560b425df64a1cbd20a46bc1d910beab7b5554903c4ee0c6d9d6d6f"[..len as usize]
    ///     );
    /// ```
    pub fn calc_hash(text: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(text.as_bytes());
        let result = hasher.finalize();
        let len: u8 = read_custom_config()
            .get_int("general.hash_len")
            .unwrap_or(4) as u8;
        use hex;
        hex::encode(&result)[..len as usize].to_string()
        // hex::encode(&result).to_string()
    }
}
