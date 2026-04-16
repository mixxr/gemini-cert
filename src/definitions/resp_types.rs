use google_ai_rs::{Client, AsSchema};
use serde::*;

#[derive(Serialize, Deserialize, AsSchema, Debug)]
pub struct StockInfo {
    certificate_isin: String,
    stock_name: String,
    stock_google_finance_ticker: String,
    stock_isin: String,
    stock_exchange: String,
    stock_sector: String,
    stock_industry: String,
    stock_tags: String,
}

#[derive(Serialize, Deserialize, AsSchema, Debug)]
pub struct CertificateTickersResponse {
    certificate_isin: String,
    certificate_issuer: String,
    pub underlyings: Vec<StockInfo>,
}

#[derive(Serialize, Deserialize, AsSchema, Debug)]
pub struct CertificateResponse {
    isin: String,
    issuer: String,
    name: String,
    type_tags: String,
    sector: String,
    industry: String,   
    stock_exchange: String,
    currency: String,
    issue_date: String,
    rembursement_date: String,
    callable: String,
    maturity_date: String,
    coupon: String,
    coupon_recurrence: String,
    issuer_rating: String,
    leverage: String,
    exchange_risk: String,
    capital_protection: String,
}