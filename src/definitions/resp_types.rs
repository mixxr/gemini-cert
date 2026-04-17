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
    stock_specializations: String, // e.g. AI, Quantum, Defense Drone, Defense communication, Satellite, Crypto, etc.
    stock_capitalization: String, // e.g. micro, small, mid, large, mega
    stock_pe: String, // e.g. P/E ratio value or N/A if not applicable
    stock_beta: String, // e.g. beta value or N/A if not applicable
    stock_volatility: String, // e.g. low, medium, high, etc.
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
    name: String, // the name of the certificate, try to add the underlying stock tickers to the name if possible
    certificate_type_tags: String, // e.g. step-down, memory, cash collect, booster, etc.
    memory_effect: String, // yes, no, etc.
    phase: String, // rembursed, active, etc.
    currency: String,
    industry: String,  // try to infer the industry of the certificate based on the underlying stocks' industries
    callable: String, // yes, no, autocallable,
    strike_date: String, // format YYYY-MM-DD
    issue_date: String, // format YYYY-MM-DD
    rembursement_date: String, // format YYYY-MM-DD
    autocallable_date: String, // format YYYY-MM-DD
    capital_barrier: String, // e.g. 100% of the strike price, 50% of the underlying stock price, etc.
    airbag: String, // yes, no, etc.
    risk_level: String, // low, medium, high, etc.
    coupon_amount: String,
    coupon_recurrence: String,
    coupon_type: String, // fixed, variable, etc.
    coupon_barrier: String, // e.g. 100% of the strike price, 50% of the underlying stock price, etc.
    leverage: String,
    exchange_risk: String,
}

#[derive(Serialize, Deserialize, AsSchema, Debug)]
pub struct IssuerInfo {
    issuer_name: String,
    specialization: String,
    geo_region: String,   
    issuer_rating_description: String, // e.g. S&P's: A (04/07/2018)
    issuer_rating_class: String, // e.g. AA, BBB, etc.
}