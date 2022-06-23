use reqwest::header::{self, HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;

use super::AccountConfig;

#[derive(Debug)]
pub struct KisRequest {
    pub req_type: RequestType,
    pub url: String,
    pub headers: HeaderMap,
    pub parameters: HashMap<String, String>,
}

#[derive(Debug)]
pub enum RequestType {
    GET,
    POST,
    POSTTOKEN,
}

impl KisRequest {
    pub fn new(req_type: RequestType, conf: &AccountConfig) -> Self {
        let url = conf.get_url();

        match req_type {
            RequestType::GET => {
                let mut headers = HeaderMap::new();
                headers.insert(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("application/json"),
                );
                headers.insert(
                    HeaderName::from_static("appkey"),
                    HeaderValue::from_str(conf.get_apikey()).unwrap(),
                );
                headers.insert(
                    HeaderName::from_static("appsecret"),
                    HeaderValue::from_str(conf.get_secret()).unwrap(),
                );

                let parameters: HashMap<String, String> = HashMap::new();

                Self {
                    req_type,
                    headers,
                    parameters,
                    url,
                }
            }
            RequestType::POST => {
                let mut headers = HeaderMap::new();
                headers.insert(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("application/json"),
                );
                headers.insert(
                    HeaderName::from_static("appkey"),
                    HeaderValue::from_str(conf.get_apikey()).unwrap(),
                );
                headers.insert(
                    HeaderName::from_static("appsecret"),
                    HeaderValue::from_str(conf.get_secret()).unwrap(),
                );

                let parameters: HashMap<String, String> = HashMap::from([]);

                Self {
                    req_type,
                    headers,
                    parameters,
                    url,
                }
            }
            RequestType::POSTTOKEN => {
                // This is for only get access-token
                let mut headers = HeaderMap::new();
                headers.insert(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("application/json"),
                );

                let parameters: HashMap<String, String> = HashMap::from([
                    ("appkey".to_string(), conf.get_apikey().to_string()),
                    ("appsecret".to_string(), conf.get_secret().to_string()),
                    ("grant_type".to_string(), "client_credentials".to_string()),
                ]);

                Self {
                    req_type,
                    headers,
                    parameters,
                    url,
                }
            }
        }
    }
}
