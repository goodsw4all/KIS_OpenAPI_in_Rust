use reqwest::blocking;
use reqwest::header::{self, HeaderName, HeaderValue};
use std::collections::HashMap;

use super::AccountConfig;
use super::{KisRequest, RequestType};

type KisResult<T> = Result<T, Box<dyn std::error::Error>>;

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
    ) -> KisResult<KisRequest> {
        let mut req = KisRequest::new(req_type, &self.account_info);
        let auth_header = format!("Bearer {}", self.account_info.get_access_token());
        req.headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(auth_header.as_str())?,
        );

        // request URL
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
                // println!("Response Headers:\n{:#?}", res.headers());
                let v: serde_json::Value = serde_json::from_str(&res.text()?)?;
                Ok(v)
            }
            s => {
                println!("Response Error : {} \n\t{:?}", s, res);
                Err("Response Error".into())
            }
        }
    }

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

    pub fn order_buy_stock(
        &self,
        ticker: &str,
        order_type: &str,
        count: u32,
        price: u32,
    ) -> KisResult<serde_json::Value> {
        self.order_stock(ticker, order_type, count, price, true)
    }

    pub fn order_sell_stock(
        &self,
        ticker: &str,
        order_type: &str,
        count: u32,
        price: u32,
    ) -> KisResult<serde_json::Value> {
        self.order_stock(ticker, order_type, count, price, false)
    }

    pub fn order_stock(
        &self,
        ticker: &str,
        order_type: &str,
        count: u32,
        price: u32,
        buy: bool,
    ) -> KisResult<serde_json::Value> {
        let url = "/uapi/domestic-stock/v1/trading/order-cash";

        let parameters = [
            ("CANO", "50067252"),
            ("ACNT_PRDT_CD", "01"),
            ("PDNO", ticker),
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

    pub fn get_ordered_list(&self) -> KisResult<serde_json::Value> {
        if !self.account_info.is_real() {
            return Err("Not Available for virtual account".into());
        }
        let url = "/uapi/domestic-stock/v1/trading/inquire-psbl-rvsecncl";

        let tr_id = "TTTC8036R";
        let headers = [("tr_id", tr_id)];

        let query = [
            ("CANO", self.account_info.get_account_no()),
            ("ACNT_PRDT_CD", "01"),
            ("CTX_AREA_FK100", ""),
            ("CTX_AREA_NK100", ""),
            ("INQR_DVSN_1", "0"),
            ("INQR_DVSN_2", "0"),
        ];

        let req = self.make_request(url, RequestType::GET, &headers, &query)?;

        self.send_request(req)
    }

    // 국내주식시세
    /// 주식현재가 시세[v1_국내주식-008]
    pub fn get_stock_current_price(&self, ticker: &str) -> KisResult<serde_json::Value> {
        let url = "/uapi/domestic-stock/v1/quotations/inquire-price";
        let headers = [("tr_id", "FHKST01010100")];
        let query = [("fid_cond_mrkt_div_code", "J"), ("fid_input_iscd", ticker)];

        let req = self.make_request(url, RequestType::GET, &headers, &query)?;

        self.send_request(req)
    }

    /// 주식현재가 체결[v1_국내주식-009]
    pub fn get_stock_current_concluded(&self, ticker: &str) -> KisResult<serde_json::Value> {
        let url = "/uapi/domestic-stock/v1/quotations/inquire-ccnl";
        let headers = [("tr_id", "FHKST01010300")];
        let query = [("fid_cond_mrkt_div_code", "J"), ("fid_input_iscd", ticker)];
        let req = self.make_request(url, RequestType::GET, &headers, &query)?;

        self.send_request(req)
    }

    /// 주식현재가 일자별[v1_국내주식-010]
    pub fn get_stock_daily_price(&self, ticker: &str) -> KisResult<serde_json::Value> {
        let url = "/uapi/domestic-stock/v1/quotations/inquire-daily-price";
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

    /// 주식현재가 호가 예상체결[v1_국내주식-011]
    pub fn get_stock_bid_ask_prices(&self, ticker: &str) -> KisResult<serde_json::Value> {
        let url = "/uapi/domestic-stock/v1/quotations/inquire-asking-price-exp-ccn";
        let headers = [("tr_id", "FHKST01010200")];
        let query = [("fid_cond_mrkt_div_code", "J"), ("fid_input_iscd", ticker)];
        let req = self.make_request(url, RequestType::GET, &headers, &query)?;

        self.send_request(req)
    }

    /// 주식현재가 투자자[v1_국내주식-012]
    pub fn get_stock_investor_list(&self, ticker: &str) -> KisResult<serde_json::Value> {
        let url = "/uapi/domestic-stock/v1/quotations/inquire-investor";
        let headers = [("tr_id", "FHKST01010900")];
        let query = [("fid_cond_mrkt_div_code", "J"), ("fid_input_iscd", ticker)];
        let req = self.make_request(url, RequestType::GET, &headers, &query)?;

        self.send_request(req)
    }

    /// 주식현재가 회원사[v1_국내주식-013]
    pub fn get_stock_membership_list(&self, ticker: &str) -> KisResult<serde_json::Value> {
        let url = "/uapi/domestic-stock/v1/quotations/inquire-member";
        let headers = [("tr_id", "FHKST01010600")];
        let query = [("fid_cond_mrkt_div_code", "J"), ("fid_input_iscd", ticker)];
        let req = self.make_request(url, RequestType::GET, &headers, &query)?;

        self.send_request(req)
    }

    /// ELW현재가 시세[v1_국내주식-014] not tested
    pub fn get_stock_elw_price(&self, ticker: &str) -> KisResult<serde_json::Value> {
        let url = "/uapi/domestic-stock/v1/quotations/inquire-elw-price";
        let headers = [("tr_id", "FHKEW15010000")];
        let query = [("fid_cond_mrkt_div_code", "W"), ("fid_input_iscd", ticker)];
        let req = self.make_request(url, RequestType::GET, &headers, &query)?;

        self.send_request(req)
    }

    /// 국내주식기간별시세(일/주/월/년)[v1_국내주식-016] R not tested
    pub fn get_stock_duration_prices(
        &self,
        ticker: &str,
        begin: &str,
        end: &str,
        duration: &str,
    ) -> KisResult<serde_json::Value> {
        let url = "/uapi/domestic-stock/v1/quotations/inquire-daily-itemchartprice";
        let headers = [("tr_id", "[실전투자]")];
        let query = [
            ("fid_cond_mrkt_div_code", "J"),
            ("fid_input_iscd", ticker),
            ("fid_input_date_1", begin),
            ("fid_input_date_2", end),
            ("fid_period_div_code", duration),
            ("FID_ORG_ADJ_PRC", "0"), // 0:수정주가 1:원주가
        ];
        let req = self.make_request(url, RequestType::GET, &headers, &query)?;

        self.send_request(req)
    }

    /// 국내주식업종기간별시세(일/주/월/년)[v1_국내주식-021] R not tested
    pub fn get_sector_duration_prices(
        &self,
        section: &str,
        begin: &str,
        end: &str,
        duration: &str,
    ) -> KisResult<serde_json::Value> {
        let url = "/uapi/domestic-stock/v1/quotations/inquire-daily-indexchartprice";
        let headers = [("tr_id", "[실전투자]")];
        let query = [
            ("fid_cond_mrkt_div_code", "U"),
            ("fid_input_iscd", section),
            ("fid_input_date_1", begin),
            ("fid_input_date_2", end),
            ("fid_period_div_code", duration),
        ];
        let req = self.make_request(url, RequestType::GET, &headers, &query)?;

        self.send_request(req)
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;
    use crate::kis::load_account_config;

    static TICKER: &'static str = "003490";

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

    /// 국내주식시세
    fn run_price_req(f: fn(&KisApi, &str) -> KisResult<serde_json::Value>, ticker: &str) -> serde_json::Value {
        let kis = setup_for_wrapper_api();

        let res = f(&kis, ticker);

        assert!(res.is_ok());

        res.unwrap()
    }

    #[test]
    fn test_get_stock_current_price() {
        let v = run_price_req(KisApi::get_stock_current_price, TICKER);
        println!("Response Text : {:#?}", v);
    }

    #[test]
    fn test_get_stock_current_concluded() {
        let v = run_price_req(KisApi::get_stock_current_concluded, TICKER);
        println!("Response Text : {:#?}", v);
    }

    #[test]
    fn test_get_stock_daily_price() {
        let v = run_price_req(KisApi::get_stock_daily_price, TICKER);
        println!("Response Text : {:#?}", v);
    }

    #[test]
    fn test_get_stock_bid_ask_prices() {
        let v = run_price_req(KisApi::get_stock_bid_ask_prices, TICKER);
        println!("Response Text : {:#?}", v);
    }

    #[test]
    fn test_get_stock_inverstor_info() {
        let v = run_price_req(KisApi::get_stock_investor_list, TICKER);
        println!("Response Text : {:#?}", v);
    }

    #[test]
    fn test_get_stock_membership_list() {
        let v = run_price_req(KisApi::get_stock_membership_list, TICKER);
        println!("Response Text : {:#?}", v);
    }

    // Stock Order
    #[test]
    fn test_account_balance() {
        let kis = setup_for_wrapper_api();

        let res = kis.get_account_balance();
        assert!(res.is_ok());
        if let Ok(v) = res {
            println!("Response Text  : {:#?}", v);
        }
    }

    #[test]
    fn test_order_buy() {
        let kis = setup_for_wrapper_api();

        let res = kis.order_buy_stock(TICKER, "01", 1, 0);
        assert!(res.is_ok());

        if let Ok(v) = res {
            println!("Response Text  : {}", v);
        }
    }

    #[test]
    fn test_order_sell() {
        let kis = setup_for_wrapper_api();

        let res = kis.order_sell_stock(TICKER, "01", 1, 0);
        assert!(res.is_ok());

        if let Ok(v) = res {
            println!("Response Text  : {}", v);
        }
    }
}
