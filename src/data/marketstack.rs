use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::stats::Statistics;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pagination {
    limit: i32,
    offset: i32,
    count: i32,
    total: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    adj_high: f64,
    adj_low: f64,
    adj_close: f64,
    adj_open: f64,
    adj_volume: f64,
    split_factor: f64,
    dividend: f64,
    symbol: String,
    exchange: String,
    date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StockData {
    pagination: Pagination,
    data: Vec<Data>,
}

impl StockData {
    /// Serialize the response from a saved file
    pub fn from_file(path: std::path::PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        serde_json::from_reader(reader).map_err(|e| e.into())
    }

    /// Return the holding period returns for the data
    pub fn holding_period_returns(&self) -> Vec<f64> {
        let mut holding_period_returns = Vec::new();

        for (i, data) in self.data.iter().enumerate() {
            if i == 0 {
                holding_period_returns.push(0.0);
            } else {
                let previous_close = self.data[i - 1].close;
                let current_close = data.close;
                let holding_period_return =
                    (current_close - previous_close + data.dividend) / previous_close;
                holding_period_returns.push(holding_period_return);
            }
        }

        holding_period_returns
    }
}

impl Statistics for StockData {
    /// Return the vector of values
    fn values(&self) -> Vec<f64> {
        self.holding_period_returns()
    }
}
