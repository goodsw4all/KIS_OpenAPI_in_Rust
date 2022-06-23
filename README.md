# KIS OpenAPI in Rust

KIS 트레이딩 오픈API Client in Rust (https://apiportal.koreainvestment.com/about)    
한국투자증권에서 Rest API / WebSocket 을 증권사 최초 공개, 보다 제약없이 stock trading tool을 만들 수 있게 됨
(기존의 API들은 windows 전용(OCX/COM)이었고, 제한적 사용)  

## KIS Developers Open API with Rust
- Python으로 초기 개발을 하다가, 주문에 있어서는 속도가 중요하므로 Rust로 재작성   
- 대부분의 주식 관련 Lib 들이 python으로 구현되어 있으므로 FFI를 통해 활용할 것
- 실시간 데이터 처리 및 매매에 대한 부분들은 모두 Rust로 구현 예정

## Goal
- Step 1 : Open API client용 SDK 개발
- Step 2 : Automatic Trading (Policy based)
- Step 3 : Strategy 

## Development 
### SDK (Working on only in virtual account for now)
#### API
- [x] Oauth 인증
  - [x] 보안인증키 발급 (access token)
  - [x] Hashkey
- [ ] 국내주식주문
  - [x] 매수 주문
  - [x] 매도 주문
  - [ ] 정정 취소 주문
- [ ] 국내주식시세
  - [x] 주식현재가 시세[v1_국내주식-008]
  - [x] 주식현재가 체결[v1_국내주식-009]
  - [x] 주식현재가 일자별[v1_국내주식-010]
  - [ ] 주식현재가 호가 예상체결[v1_국내주식-011]
  - [x] 주식현재가 투자자[v1_국내주식-012]
  - [ ] 주식현재가 회원사[v1_국내주식-013]
  - [ ] ELW현재가 시세[v1_국내주식-014]
  - [ ] 국내주식기간별시세(일/주/월/년)[v1_국내주식-016]
  - [ ] 국내주식업종기간별시세(일/주/월/년)[v1_국내주식-021]
- [ ] Websockets
  - [x] 주식호가 (활용방법에 따라 수정 필요)
  - [ ] 체결통보
- [ ] 해외주식주문 
  - [ ] TBD
- [ ] 해외주식현재가
  - [ ] TBD
### Code Generation
#### API Tempalte 생성
  - [x] Excel API 문서를 읽고 API Template 자동 생성 [code_gen.py](./docs/code_gen.py)

### Automatic Trading
- [ ] TBD
### Strategy
- [ ] TBD

## Documentation

## References
### KIS Portal https://apiportal.koreainvestment.com/
### Rust https://doc.rust-lang.org/book/
