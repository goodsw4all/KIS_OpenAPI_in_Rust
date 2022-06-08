use reqwest::blocking;
use reqwest::header::{self, HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;

use super::account::AccountConfig;

type KisResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
struct KisRequest {
    url: String,
    headers: HeaderMap,
    parameters: HashMap<String, String>,
}

enum RequestType {
    GET,
    POST,
    POSTTOKEN,
}

impl KisRequest {
    fn new(req_type: RequestType, conf: &AccountConfig) -> Self {
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
                    headers,
                    parameters,
                    url,
                }
            }
            RequestType::POSTTOKEN => {
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
                    headers,
                    parameters,
                    url,
                }
            }
        }
    }
}

pub struct KisApi {
    account_info: AccountConfig,
}

impl KisApi {
    pub fn new(account_info: AccountConfig) -> Self {
        Self { account_info }
    }

    pub fn get_hashkey(&self, parameters: &[(&str, &str)]) -> KisResult<serde_json::Value> {
        let url = "/uapi/hashkey";
        let headers = [];

        let req = self.make_request(url, RequestType::POST, &headers, parameters)?;

        // println!("{:?}", req.headers);
        self.send_post_request(req)
    }

    pub fn issue_access_token(&mut self) -> KisResult<bool> {
        if self.account_info.is_acces_token_valid() {
            return Ok(true);
        }
        let kis_request = KisRequest::new(RequestType::POSTTOKEN, &self.account_info);

        let client = reqwest::blocking::Client::new();
        let res: blocking::Response = client
            .post(kis_request.url + "/oauth2/tokenP")
            .headers(kis_request.headers)
            .json(&kis_request.parameters)
            .send()?;

        if res.status() == reqwest::StatusCode::OK {
            let v: serde_json::Value = serde_json::from_str(&res.text()?)?;
            self.account_info
                .set_access_token(v["access_token"].as_str().unwrap());
            // println!("{:?}", self.account_info);
            return Ok(true);
        } else {
            println!("Request error {}", res.status());
        }

        Ok(false)
    }

    fn make_request(
        &self,
        url: &str,
        req_type: RequestType,
        headers: &[(&str, &str)],
        parameters: &[(&str, &str)],
    ) -> KisResult<KisRequest> {
        let mut req = KisRequest::new(req_type, &self.account_info);
        let auth_header = format!("Bearer {}", self.account_info.get_access_token());
        req.headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(auth_header.as_str())?,
        );

        // URL
        req.url = req.url + url;

        // additional headers
        for (k, v) in headers {
            req.headers
                .insert(k.parse::<HeaderName>().unwrap(), v.parse().unwrap());
        }

        // query
        for (k, v) in parameters {
            req.parameters.insert(k.to_string(), v.to_string());
        }

        // println!("{:?}", req.headers);

        Ok(req)
    }

    fn send_get_request(&self, req: KisRequest) -> KisResult<serde_json::Value> {
        let client = reqwest::blocking::Client::new();

        let res: blocking::Response = client
            .get(req.url)
            .headers(req.headers)
            .query(&req.parameters)
            .send()?;

        //TODO:
        let v: serde_json::Value = serde_json::from_str(&res.text()?)?;
        Ok(v)
    }

    fn send_post_request(&self, req: KisRequest) -> KisResult<serde_json::Value> {
        let client = reqwest::blocking::Client::new();

        let res: blocking::Response = client
            .post(req.url)
            .headers(req.headers)
            .json(&req.parameters)
            .send()?;

        //TODO:
        println!("{}", res.status());
        let v: serde_json::Value = serde_json::from_str(&res.text()?)?;
        Ok(v)
    }

    pub fn get_stock_price_days(&self, ticker: &str) -> KisResult<serde_json::Value> {
        let url = "/uapi/domestic-stock/v1/quotations/inquire-price";
        let headers = [("tr_id", "FHKST01010400")];
        let query = [
            ("fid_cond_mrkt_div_code", "J"),
            ("fid_input_iscd", ticker),
            ("fid_org_adj_prc", "1"),
            ("fid_period_div_code", "D"),
        ];

        let req = self.make_request(url, RequestType::GET, &headers, &query)?;

        self.send_get_request(req)
    }

    pub fn get_stock_price_realtime(&self, ticker: &str) -> KisResult<serde_json::Value> {
        let url = "/uapi/domestic-stock/v1/quotations/inquire-price";
        let headers = [("tr_id", "FHKST01010100")];
        let query = [("fid_cond_mrkt_div_code", "J"), ("fid_input_iscd", ticker)];

        let req = self.make_request(url, RequestType::GET, &headers, &query)?;

        self.send_get_request(req)
    }
}

/// Attempts to try reqwest crate
/// send `GET` request to httpbin.org which is a simple HTTP Request & Response service.
/// this test gets a json response including your external IP address
fn _get_reqwest_practice() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let res: blocking::Response = client
        .get("https:/www.httpbin.org/get")
        .header(header::ACCEPT, "application/json")
        .header(header::USER_AGENT, "hyper/0.5.2".to_owned())
        .send()?;

    if res.status() == reqwest::StatusCode::OK {
        let v: serde_json::Value = serde_json::from_str(&res.text()?)?;
        println!("Response Text: {}", v["origin"]);
        return Ok(v["origin"].to_string());
    }

    Err("Not Reach Here".into())
}

#[cfg(test)]
mod tests {
    use crate::kis::load_account_config;

    use super::*;

    #[test]
    fn test_load_account_config_all_valid() {
        let conf = load_account_config("./secret");
        let empty_acc_info = AccountConfig::new();
        let conf = conf.unwrap_or(AccountConfig::new());

        assert_ne!(conf, empty_acc_info);
    }

    #[test]
    fn test_issue_request_token() {
        let mut kis = KisApi::new(load_account_config("./secret").unwrap());
        let res = kis.issue_access_token();
        assert!(res.is_ok())
    }

    #[test]
    fn test_get_price_realtime() {
        let mut kis = KisApi::new(load_account_config("./secret").unwrap());
        let res = kis.issue_access_token();
        let v = kis.get_stock_price_realtime("069960");
        assert!(res.is_ok());
        assert!(v.is_ok());
        if let Ok(v) = v {
            println!("Response Text 주식 현재가 : {}", v["output"]["stck_prpr"]);
        }
    }

    #[test]
    fn test_get_price_days() {
        let mut kis = KisApi::new(load_account_config("./secret").unwrap());
        let res = kis.issue_access_token();
        let v = kis.get_stock_price_days("069960");
        assert!(res.is_ok());
        assert!(v.is_ok());
        if let Ok(v) = v {
            println!("Response Text 주식 현재가 : {}", v);
        }
    }

    #[test]
    fn test_get_hashkey() {
        let kis = KisApi::new(load_account_config("./secret").unwrap());
        let parameters = [
            ("CANO", "00000000"),
            ("ACNT_PRDT_CD", "01"),
            ("OVRS_EXCG_CD", "SHAA"),
            ("PDNO", "00001"),
            ("ORD_QTY", "500"),
            ("OVRS_ORD_UNPR", "52.65"),
            ("ORD_SVR_DVSN_CD", "0"),
        ];

        let v = kis.get_hashkey(&parameters);

        assert!(v.is_ok());
        if let Ok(v) = v {
            println!("Response Text : {}", v);
        }
    }

    #[test]
    #[ignore = "just for checking wheter reqwest is working"]
    fn practice_reqwest() {
        let res = _get_reqwest_practice();
        assert!(res.is_ok())
    }
}
