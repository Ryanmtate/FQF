use std::{collections::HashMap, f32::consts::E};

use crate::{portfolio::Ticker, stats::Statistics};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Pagination {
    limit: i32,
    offset: i32,
    count: i32,
    total: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub adj_high: f64,
    pub adj_low: f64,
    pub adj_close: f64,
    pub adj_open: f64,
    pub adj_volume: f64,
    pub split_factor: f64,
    pub dividend: f64,
    pub symbol: String,
    pub exchange: String,
    pub date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct StockData {
    pub pagination: Pagination,
    pub data: Vec<Data>,
}

// Stock Price Return data
#[derive(Debug, Clone, Default)]
pub struct Returns(pub Vec<f64>);

impl StockData {
    /// Serialize the response from a saved file
    pub fn from_file(path: std::path::PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        serde_json::from_reader(reader).map_err(|e| e.into())
    }

    pub async fn download(
        tickers: Vec<Ticker>,
        num_days: usize,
    ) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        if tickers.is_empty() {
            return Err("No tickers provided".into());
        }

        if num_days == 0 {
            return Err("No days provided".into());
        }

        let mut data = vec![];

        println!("API KEY: {:?}", api_key);

        let days_per_ticker = 1000 / tickers.len();
        let mut num_requests = num_days / days_per_ticker;
        let mut offset = 0;

        while num_requests > 0 {
            let url = format!(
                "http://api.marketstack.com/v1/eod?access_key={}&symbols={}&limit=1000&offset={}",
                api_key,
                tickers.join(","),
                offset
            );

            let results: Self = reqwest::get(url).await?.json().await?;

            data.push(results);

            offset += 1000;
            num_requests -= 1;
        }

        Ok(data)
    }

    pub fn returns(&self) -> Returns {
        let mut values = vec![];

        let mut current = 0.0;
        let mut prior = 0.0;

        for (i, v) in self.data.iter().enumerate() {
            if i == 0 {
                values.push(0.0)
            } else {
                current = self.data[i].close;
                prior = self.data[i - 1].close;
                let dividend = self.data[i].dividend;

                let hpy = (current - prior + dividend) / prior;

                values.push(hpy)
            }
        }

        Returns(values)
    }
}

impl Statistics for Returns {
    fn values(&self) -> Vec<f64> {
        self.0.clone()
    }
}
