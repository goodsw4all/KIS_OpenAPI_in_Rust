# KIS OpenAPI in Rust

KIS 트레이딩 오픈API Client in Rust (https://apiportal.koreainvestment.com/about)    
한국투자증권에서 Rest API 를 증권사 최초 공개, 보다 자유롭게 stock trading tool을 만들 수 있게 됨  
(기존의 API들은 windows 전용(OCX/COM)이었고, 사용이 불편)  

## KIS Developers Open API with Rust
- Python으로 개발을 시작했지만, 최근 공부 중인 Rust로 변경하여 구현 시작  
- 대부분의 주식 관련 Lib 들이 python으로 구현되어 있으므로 FFI를 통해 활용할 것
- 실시간 데이터 처리 및 매매에 대한 부분들은 모두 Rust로 구현 예정

## Goal
- Step 1 : Open API client용 SDK 개발
- Step 2 : Automatic Trading (Policy based)
- Step 3 : Strategy 

## Development 
### SDK (Working on only in virtual account for now)
#### API
- [x] 보안인증키 발급 (access token)
- [x] Hashkey
- [x] 현재가 시세
- [x] 주식 현재가 일자별
- [x] 매수 주문
- [x] 매도 주문
- [ ] 정정 취소 주문
- [ ] TBD 
#### Websockets
- [ ] 주식호가
- [ ] 체결통보
- [ ] TBD 

### Automatic Trading

### Strategy
