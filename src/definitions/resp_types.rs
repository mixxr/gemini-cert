use google_ai_rs::{AsSchema};
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
    pub details: Option<CertificateDetails>,
    pub underlyings: Option<Vec<StockInfo>>,
    pub issuer: Option<IssuerInfo>,
}

#[derive(Serialize, Deserialize, AsSchema, Debug)]
pub struct CertificateDetails {
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
    coupon_amount: String,
    coupon_recurrence: String,
    leverage: String,
    exchange_risk: String,
    capital_protection: String,
}

#[derive(Serialize, Deserialize, AsSchema, Debug)]
pub struct IssuerInfo {
    issuer_name: String,
    specialization: String,
    geo_region: String,   
    issuer_rating_description: String, // e.g. S&P's: A (04/07/2018)
    issuer_rating_class: String, // e.g. AA, BBB, etc.
}