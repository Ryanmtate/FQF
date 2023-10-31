use crate::bond::{self, Bond, Frequency};
use crate::methods::{internal_rate_of_return as irr, net_present_value};
use crate::portfolio::Portfolio;
use crate::stats::Statistics;
use crate::stock_data::StockData;

use chrono::{Months, Utc};

const PORTFOLIO_PATH: &str = "data/portfolio.json";

#[test]
pub fn test_bond_issuance() -> Result<(), bond::Error> {
    let par_value = 1_000.0;

    let annual_interest_rate = 0.06;

    // 60 months or five years;
    let maturity_date = Utc::now()
        .checked_add_months(Months::new(12 * 3))
        .ok_or(bond::Error::InvalidMaturityDate)?;
    let frequency = Frequency::SemiAnnual;

    let bond = Bond::issue(par_value, annual_interest_rate, frequency, maturity_date)?;

    let market_price = Some(par_value);

    assert!(bond.issuance_date <= Utc::now());

    let payment = bond.coupon_payment();
    println!("Payment: {:?}", payment);

    let periodic_rate = bond.periodic_rate();
    println!("Periodic Rate: {:?}", periodic_rate);

    let num_periods = bond.compounding_periods();
    println!("Number of Periods: {:?}", num_periods);

    let future_value = bond.future_value();
    println!("Future Value: {:?}", future_value);

    let yield_to_maturity = bond.yield_to_maturity();
    println!("Yield to Maturity: {:?}", yield_to_maturity);

    let present_value = bond.present_value();
    println!("Present Value: {:?}", present_value);

    let annual_cash_flow = bond.annual_cash_flow();
    println!("Annual Cash Flow: {:?}", annual_cash_flow);

    let current_yield = bond.current_yield();
    println!("Current Yield: {:?}", current_yield);

    let duration = bond.duration(market_price);
    println!("Duration: {:?}", duration);

    let modified_duration = bond.modified_duration(market_price);
    println!("Modified Duration: {:?}", modified_duration);

    let cash_flows = bond.cash_flows();
    println!("Cash Flows: {:?}", cash_flows);

    let npv = net_present_value(present_value, cash_flows.clone(), bond.periodic_rate());

    println!("NPV: {:?}", npv);

    let irr = irr(present_value, cash_flows);

    println!("IRR: {:?}", irr);

    Ok(())
}

#[test]
pub fn test_stock_data() -> Result<(), Box<dyn std::error::Error>> {
    let path =
        std::path::PathBuf::from("/Users/ryan/Projects/fundamental_quant_finance/data/AAPL.json");
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);

    let data: crate::stock_data::StockData = serde_json::from_reader(reader)?;

    // println!("Data: {:?}", data);

    let returns = data.returns();

    println!("Returns: {:?}", returns);

    let mean = returns.geometric_mean();

    println!("Mean Return: {:?}", mean);

    let skewness = returns.skewness();

    println!("Skewness: {:?}", skewness);

    let kurtosis = returns.excess_kurtosis();

    println!("Kurtosis: {:?}", kurtosis);

    let neg_returns = returns.0.iter().filter(|r| *r < &0.0).count();

    let prob_negative = neg_returns as f64 / returns.0.len() as f64;

    println!("Probability of Negative Return: {:?}", prob_negative);

    let range = returns.range();

    println!("Range: {:?}", range);

    let max = returns.max();
    let min = returns.min();

    println!("Max: {:?}", max);
    println!("Min: {:?}", min);

    // What is the probability of the returns between 0.02 & 0.05
    let min_bound = 0.02;
    let max_bound = 0.05;

    let prob_bound = returns
        .0
        .iter()
        .filter(|r| *r >= &min_bound && *r <= &max_bound)
        .count() as f64
        / returns.0.len() as f64;

    println!(
        "Probability of Return Between {:?} and {:?}: {:?}",
        min_bound, max_bound, prob_bound
    );

    let expected_p = returns.expected_probability();

    println!("Expected Probability: {:?}", expected_p);

    Ok(())
}

#[tokio::test]
async fn test_stock_data_download() -> Result<(), Box<dyn std::error::Error>> {
    let tickers = vec![
        "AAPL".to_string(),
        "MSFT".to_string(),
        "GOOG".to_string(),
        "AMZN".to_string(),
        "META".to_string(),
    ];

    let data = StockData::download(tickers, 400).await?;

    println!("Data: {:?}", data);

    Ok(())
}

#[tokio::test]
async fn test_portfolio_assets() -> Result<(), Box<dyn std::error::Error>> {
    let tickers = vec![
        "AAPL".to_string(),
        "MSFT".to_string(),
        "GOOG".to_string(),
        "AMZN".to_string(),
        "META".to_string(),
    ];

    let mut portfolio = Portfolio::new(1e6, None);

    portfolio.download_asset_data(tickers, 400).await?;

    println!("Portfolio: {:?}", portfolio);

    let mut data_length = 0;
    for (ticker, asset) in portfolio.assets.iter() {
        if data_length == 0 {
            data_length = asset.stock_data.data.len();
            continue;
        }

        assert_eq!(data_length, asset.stock_data.data.len());
    }

    println!("Data Length: {:?}", data_length);

    portfolio.save(PORTFOLIO_PATH.into())?;

    Ok(())
}

#[tokio::test]
async fn test_portfolio_load() -> Result<(), Box<dyn std::error::Error>> {
    let mut portfolio = Portfolio::load(PORTFOLIO_PATH.into())?;

    println!("Portfolio: {:?}", portfolio);

    Ok(())
}
