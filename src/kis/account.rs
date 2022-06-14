type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

use std::fs;
use std::io;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AccountConfig {
    id: String,
    // password: String,
    real: bool,
    key: String,
    account: String,
    phone: String,
    url: String,
    ops: String,
    secret: String,
    token: String,
}

impl AccountConfig {
    pub fn new() -> Self {
        AccountConfig {
            id: "".to_string(),
            // password: "".to_string(),
            real: false,
            key: "".to_string(),
            account: "".to_string(),
            phone: "".to_string(),
            url: "".to_string(),
            ops: "".to_string(),
            secret: "".to_string(),
            token: "".to_string(),
        }
    }

    pub fn is_real(&self) -> bool {
        self.real
    }

    pub fn get_apikey(&self) -> &str {
        &self.key
    }

    pub fn get_secret(&self) -> &str {
        &self.secret
    }

    pub fn get_url(&self) -> String {
        self.url.clone()
    }

    pub fn get_access_token(&self) -> &str {
        &self.token
    }

    pub fn set_access_token(&mut self, token: &str) {
        self.token = token.to_string();
    }

    pub fn is_acces_token_valid(&self) -> bool {
        //TODO: check if expired
        self.token != ""
    }

    pub(crate) fn get_account_no(&self) -> &str {
        &self.account
    }
}

pub fn load_account_config(path: &str, real: bool) -> MyResult<AccountConfig> {
    let config_path = if real {
        format!("./{path}/kis_real.json")
    } else {
        format!("./{path}/kis_test.json")
    };

    let reader = io::BufReader::new(fs::File::open(config_path)?);
    let conf: AccountConfig = serde_json::from_reader(reader)?;

    Ok(conf)
}
