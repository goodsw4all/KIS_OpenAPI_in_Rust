use reqwest::blocking;
use reqwest::header::{self, HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;

use super::account::AccountConfig;

type KisResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
struct KisRequest {
    req_type: RequestType,
    url: String,
    headers: HeaderMap,
    parameters: HashMap<String, String>,
}

#[derive(Debug)]
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

pub struct KisApi {
    account_info: AccountConfig,
}

impl KisApi {
    pub fn new(account_info: AccountConfig) -> Self {
        Self { account_info }
    }

    pub fn get_hashkey(&self, parameters: &[(&str, &str)]) -> KisResult<(HashMap<String, String>, String)> {
        let url = "/uapi/hashkey";
        let headers = [];

        let req = self.make_request(url, RequestType::POST, &headers, parameters)?;

        let ret = req.parameters.clone();
        let v = self.send_request(req);
        if let Ok(val) = v {
            return Ok((ret, val["HASH"].to_string().trim_matches('"').to_string()));
        }

        Err("".into())
    }

    pub fn issue_access_token(&mut self) -> KisResult<bool> {
        if self.account_info.is_acces_token_valid() {
            return Ok(true);
        }
        let url = "/oauth2/tokenP";
        let headers = [];
        let parameters = [];

        let req = self.make_request(url, RequestType::POSTTOKEN, &headers, &parameters)?;

        let v = self.send_request(req);
        match v {
            Ok(json_data) => {
                self.account_info
                    .set_access_token(json_data["access_token"].as_str().unwrap());
                Ok(true)
            }
            Err(_) => Ok(false),
        }
    }

    fn make_request(
        &self,
        url: &str,
        req_type: RequestType,
        headers: &[(&str, &str)],
        parameters: &[(&str, &str)],
        // hashmap : Option<HashMap<String, String>>
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

        // parameters : query or body
        for (k, v) in parameters {
            req.parameters.insert(k.to_string(), v.to_string());
        }

        Ok(req)
    }

    fn make_request_hashkey(
        &self,
        url: &str,
        req_type: RequestType,
        headers: &[(&str, &str)],
        body: HashMap<String, String>,
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

        // parameters : query or body
        req.parameters = body;

        Ok(req)
    }

    fn send_request(&self, req: KisRequest) -> KisResult<serde_json::Value> {
        let client = reqwest::blocking::Client::new();
        let res: blocking::Response;

        res = if let RequestType::GET = req.req_type {
            client
                .get(req.url)
                .headers(req.headers)
                .query(&req.parameters)
                .send()?
        } else {
            client
                .post(req.url)
                .headers(req.headers)
                .json(&req.parameters)
                .send()?
        };

        //TODO: Error handling
        match res.status() {
            reqwest::StatusCode::OK => {
                let v: serde_json::Value = serde_json::from_str(&res.text()?)?;
                Ok(v)
            }
            s => {
                println!("Response Error : {} \n\t{:?}", s, res);
                Err("Response Error".into())
            }
        }
    }

    // 국내주식시세
    pub fn get_stock_price_days(&self, ticker: &str) -> KisResult<serde_json::Value> {
        let url = "/uapi/domestic-stock/v1/quotations/inquire-price";
        let headers = [("tr_id", "FHKST01010400")];
        let query = [
            ("fid_cond_mrkt_div_code", "J"),
            ("fid_input_iscd", ticker),
            ("fid_org_adj_prc", "1"),
            ("fid_period_div_code", "D"), // D,W,M
        ];

        let req = self.make_request(url, RequestType::GET, &headers, &query)?;

        self.send_request(req)
    }

    pub fn get_stock_price_realtime(&self, ticker: &str) -> KisResult<serde_json::Value> {
        let url = "/uapi/domestic-stock/v1/quotations/inquire-price";
        let headers = [("tr_id", "FHKST01010100")];
        let query = [("fid_cond_mrkt_div_code", "J"), ("fid_input_iscd", ticker)];

        let req = self.make_request(url, RequestType::GET, &headers, &query)?;

        self.send_request(req)
    }
    // 국내주식시세

    pub fn get_account_balance(&self) -> KisResult<serde_json::Value> {
        let url = "/uapi/domestic-stock/v1/trading/inquire-balance";

        let tr_id = if self.account_info.is_real() {
            "TTTC8434R"
        } else {
            "VTTC8434R"
        };
        let headers = [("tr_id", tr_id)];

        let query = [
            ("CANO", self.account_info.get_account_no()),
            ("ACNT_PRDT_CD", "01"),
            ("AFHR_FLPR_YN", "N"),
            ("FNCG_AMT_AUTO_RDPT_YN", "N"),
            ("FUND_STTL_ICLD_YN", "N"),
            ("INQR_DVSN", "01"),
            ("OFL_YN", "N"),
            ("PRCS_DVSN", "01"),
            ("UNPR_DVSN", "01"),
            ("CTX_AREA_FK100", ""),
            ("CTX_AREA_NK100", ""),
        ];

        let req = self.make_request(url, RequestType::GET, &headers, &query)?;

        self.send_request(req)
    }

    pub fn order_buy_stock(&self, order_type: &str, count: u32, price: u32) -> KisResult<serde_json::Value> {
        self.order_stock(order_type, count, price, true)
    }

    pub fn order_sell_stock(&self, order_type: &str, count: u32, price: u32) -> KisResult<serde_json::Value> {
        self.order_stock(order_type, count, price, false)
    }

    pub fn order_stock(&self, order_type: &str, count: u32, price: u32, buy: bool) -> KisResult<serde_json::Value> {
        let url = "/uapi/domestic-stock/v1/trading/order-cash";

        let parameters = [
            ("CANO", "50067252"),
            ("ACNT_PRDT_CD", "01"),
            ("PDNO", "005930"),
            ("ORD_DVSN", order_type),
            ("ORD_QTY", &count.to_string()),
            ("ORD_UNPR", &price.to_string()),
            // ("CTAC_TLNO", ""),
            // ("SLL_TYPE", "01"),
            ("ALGO_NO", ""),
        ];

        let hash_data = self.get_hashkey(&parameters)?;

        let tr_id = if buy {
            if self.account_info.is_real() {
                "TTTC0802U"
            } else {
                "VTTC0802U"
            }
        } else {
            if self.account_info.is_real() {
                "TTTC0801U"
            } else {
                "VTTC0801U"
            }
        };
        let headers = [
            ("custtype", "P"),
            ("tr_id", tr_id),
            ("hashkey", hash_data.1.trim_matches('"')),
        ];

        let req = self.make_request_hashkey(url, RequestType::POST, &headers, hash_data.0)?;
        self.send_request(req)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kis::load_account_config;

    fn setup_for_wrapper_api() -> KisApi {
        let mut kis = KisApi::new(load_account_config("./secret", false).unwrap());
        let res = kis.issue_access_token();
        assert_eq!(res.unwrap(), true);
        kis
    }

    #[test]
    fn test_load_account_config_all_valid() {
        let conf = load_account_config("./secret", false);
        let empty_acc_info = AccountConfig::new();
        let conf = conf.unwrap_or(AccountConfig::new());

        assert_ne!(conf, empty_acc_info);
    }

    #[test]
    fn test_get_hashkey() {
        let kis = setup_for_wrapper_api();
        let parameters = [
            ("CANO", "00000000"),
            ("ACNT_PRDT_CD", "01"),
            ("PDNO", "005930"),
            ("ORD_DVSN", "01"),
            ("ORD_QTY", "10"),
            ("ORD_UNPR", "0"),
        ];

        let v = kis.get_hashkey(&parameters);
        assert!(v.is_ok());

        if let Ok(v) = v {
            println!("hashkey {}\nmdata,  : {:?} ", v.1, v.0);
        }
    }

    #[test]
    fn test_issue_request_token() {
        let mut kis = setup_for_wrapper_api();
        let res = kis.issue_access_token();
        assert!(res.is_ok())
    }

    #[test]
    fn test_get_price_realtime() {
        let kis = setup_for_wrapper_api();

        let res = kis.get_stock_price_realtime("064350");
        assert!(res.is_ok());
        if let Ok(v) = res {
            println!("Response Text 주식 현재가 : {}", v["output"]["stck_prpr"]);
        }
    }

    #[test]
    fn test_get_price_days() {
        let kis = setup_for_wrapper_api();

        let res = kis.get_stock_price_days("069960");
        assert!(res.is_ok());
        if let Ok(v) = res {
            println!("Response Text 주식 현재가 : {}", v);
        }
    }

    #[test]
    fn test_account_balance() {
        let kis = setup_for_wrapper_api();

        let res = kis.get_account_balance();
        assert!(res.is_ok());
        if let Ok(v) = res {
            println!("Response Text  : {}", v);
        }
    }

    #[test]
    fn test_order_buy() {
        let kis = setup_for_wrapper_api();

        let res = kis.order_buy_stock("01", 1, 0);
        assert!(res.is_ok());

        if let Ok(v) = res {
            println!("Response Text  : {}", v);
        }
    }

    #[test]
    fn test_order_sell() {
        let kis = setup_for_wrapper_api();

        let res = kis.order_sell_stock("01", 1, 0);
        assert!(res.is_ok());

        if let Ok(v) = res {
            println!("Response Text  : {}", v);
        }
    }
}
