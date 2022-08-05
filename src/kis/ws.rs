use super::account::AccountConfig;

use serde_json::json;
use tungstenite::{connect, Message};
use url::Url;

use std::sync::mpsc::Sender;
// use std::{sync::mpsc, thread};

// type KisResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn websoket_test(conf: &AccountConfig, ticker: &str, tx: Sender<String>) {
    let params = json!({
      "header": {
        "appkey": conf.get_apikey(),
        "appsecret": conf.get_secret(),
        "custtype": "P",
        "tr_type": "1",
        "content-type": "utf-8"
      },
      "body": {
        "input": {
          "tr_id": "H0STASP0",
          "tr_key": ticker
        }
      }
    });

    println!("Req Header {} \nBody {}", params["header"], params["body"]);

    // let url = 'ws://ops.koreainvestment.com:21000' //실전투자
    let url = "ws://ops.koreainvestment.com:31000"; //모의투자
    let (mut socket, response) = connect(Url::parse(url).unwrap()).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");

    for (ref header, _value) in response.headers() {
        println!("* {:?}", header);
    }

    socket
        .write_message(Message::Text(params.to_string()))
        .unwrap();

    loop {
        println!("Loop In");
        let msg = socket
            .read_message()
            .expect("Error reading message");
        // println!("Received: {}", msg);

        let data = msg.to_string();
        let data = data.split('|').collect::<Vec<&str>>();

        if data[0] == "0" {
            let values = data[3]
                .split('^')
                .collect::<Vec<&str>>();
            println!("{:?}", values);
            print_order_book(&values);
        }

        // print_order_book(&msg.to_string());
        // ?tx.send(format!("Received: {}", msg));
    }
    // socket
}

fn print_order_book(recvvalue: &[&str]) {
    println!("유가증권 단축 종목코드 {}", recvvalue[0]);
    println!(
        "영업시간 [{}] 시간구분코드 [{}]",
        recvvalue[1], recvvalue[2]
    );
    println!("======================================");
    println!("매도호가10 {}    잔량10 {}", recvvalue[12], recvvalue[32]);
    println!("매도호가09 {}    잔량09 {}", recvvalue[11], recvvalue[31]);
    println!("매도호가08 {}    잔량08 {}", recvvalue[10], recvvalue[30]);
    println!("매도호가07 {}    잔량07 {}", recvvalue[9], recvvalue[29]);
    println!("매도호가06 {}    잔량06 {}", recvvalue[8], recvvalue[28]);
    println!("매도호가05 {}    잔량05 {}", recvvalue[7], recvvalue[27]);
    println!("매도호가04 {}    잔량04 {}", recvvalue[6], recvvalue[26]);
    println!("매도호가03 {}    잔량03 {}", recvvalue[5], recvvalue[25]);
    println!("매도호가02 {}    잔량02 {}", recvvalue[4], recvvalue[24]);
    println!("매도호가01 {}    잔량01 {}", recvvalue[3], recvvalue[23]);
    println!("------------------------------");
    println!("매수호가01 {}    잔량01 {}", recvvalue[13], recvvalue[33]);
    println!("매수호가02 {}    잔량02 {}", recvvalue[14], recvvalue[34]);
    println!("매수호가03 {}    잔량03 {}", recvvalue[15], recvvalue[35]);
    println!("매수호가04 {}    잔량04 {}", recvvalue[16], recvvalue[36]);
    println!("매수호가05 {}    잔량05 {}", recvvalue[17], recvvalue[37]);
    println!("매수호가06 {}    잔량06 {}", recvvalue[18], recvvalue[38]);
    println!("매수호가07 {}    잔량07 {}", recvvalue[19], recvvalue[39]);
    println!("매수호가08 {}    잔량08 {}", recvvalue[20], recvvalue[40]);
    println!("매수호가09 {}    잔량09 {}", recvvalue[21], recvvalue[41]);
    println!("매수호가10 {}    잔량10 {}", recvvalue[22], recvvalue[42]);
    println!("======================================");
    println!("총매도호가 잔량        {}", recvvalue[43]);
    println!("총매도호가 잔량 증감    {}", recvvalue[54]);
    println!("총매수호가 잔량        {}", recvvalue[44]);
    println!("총매수호가 잔량 증감    {}", recvvalue[55]);
    println!("시간외 총매도호가 잔량   {}", recvvalue[45]);
    println!("시간외 총매수호가 증감   {}", recvvalue[46]);
    println!("시간외 총매도호가 잔량   {}", recvvalue[56]);
    println!("시간외 총매수호가 증감   {}", recvvalue[57]);
    println!("예상 체결가           {}", recvvalue[47]);
    println!("예상 체결량           {}", recvvalue[48]);
    println!("예상 거래량           {}", recvvalue[49]);
    println!("예상체결 대비          {}", recvvalue[50]);
    println!("부호                 {}", recvvalue[51]);
    println!("예상체결 전일대비율    {}", recvvalue[52]);
    println!("누적거래량            {}", recvvalue[53]);
    println!("주식매매 구분코드     {}", recvvalue[58]);
}

#[cfg(test)]
mod unit_websoket {
    use std::{sync::mpsc, thread};

    use crate::kis::load_account_config;

    use super::websoket_test;

    #[test]
    #[ignore]
    fn test_ws_connect() {
        let (tx, rx) = mpsc::channel::<String>();
        let conf = load_account_config("./secret", false).unwrap();

        let thread_join_handle = thread::spawn(move || {
            // some work herewebsoket_test(&conf, "005935");
            websoket_test(&conf, "005935", tx);
        });

        let received = rx.recv().unwrap();

        let res = thread_join_handle.join();
    }
}
