use chrono::{DateTime, Utc};

/// Error type for bond instrument
#[derive(Debug)]
pub enum Error {
    /// Invalid maturity date
    InvalidMaturityDate,
}

#[derive(Clone)]
/// Frequency of the compounding period
pub enum Frequency {
    Annual,
    SemiAnnual,
    Quarterly,
    Monthly,
    Weekly,
    Daily,
}

impl Frequency {
    /// Convert the frequency to a f64
    pub fn to_f64(&self) -> f64 {
        match self {
            Frequency::Annual => 1.0,
            Frequency::SemiAnnual => 2.0,
            Frequency::Quarterly => 4.0,
            Frequency::Monthly => 12.0,
            Frequency::Weekly => 52.0,
            Frequency::Daily => 365.0,
        }
    }
}

/// Representation of a fixed-income (debt) bond instrument
pub struct Bond {
    /// Utc date time for the maturity date
    pub maturity_date: DateTime<Utc>,
    /// Utc date time for the issuance date
    pub issuance_date: DateTime<Utc>,
    /// Frequency of the compounding period
    pub frequency: Frequency,
    /// Current or present value of the bond
    pub par_value: f64,
    /// Annual interest rate of the bond
    pub annual_interest_rate: f64,
}

impl Bond {
    /// Method for issuing a bond with number of periods, periodic rate, and present value
    pub fn issue(
        par_value: f64,
        annual_interest_rate: f64,
        frequency: Frequency,
        maturity_date: DateTime<Utc>,
    ) -> Result<Self, Error> {
        // Get the maturity year based on current year + period years.
        let issuance_date = Utc::now();

        // Ensure the maturity date is greater than the issuance date by at least the frequency.
        if maturity_date <= issuance_date {
            return Err(Error::InvalidMaturityDate);
        }

        // Create the bond.
        let bond = Self {
            issuance_date,
            maturity_date,
            frequency,
            par_value,
            annual_interest_rate,
        };

        // Ensure the bond has at least one compounding period.
        if bond.compounding_periods() < 1.0 {
            return Err(Error::InvalidMaturityDate);
        }

        Ok(bond)
    }

    /// Periodic interest rate of the bond
    pub fn periodic_rate(&self) -> f64 {
        self.annual_interest_rate / self.frequency.to_f64()
    }

    /// Calculates the term to maturity of the bond measured in years
    pub fn term_to_maturity(&self) -> f64 {
        // Ensure the maturity date is greater than the issuance date.
        if self.maturity_date <= self.issuance_date {
            return 0.0;
        }

        self.maturity_date
            .signed_duration_since(self.issuance_date)
            .num_days() as f64
            // Convert days to years
            / Frequency::Daily.to_f64()
    }

    /// Number of compounding periods of the bond based on the term to maturity and the coupon frequency
    pub fn compounding_periods(&self) -> f64 {
        self.term_to_maturity() * self.frequency.to_f64()
    }

    /// Calculate the coupon payment of the bond
    pub fn coupon_payment(&self) -> f64 {
        self.par_value * self.periodic_rate()
    }

    /// Calculate the future value of the bond
    pub fn future_value(&self) -> f64 {
        self.par_value * (1.0 + self.periodic_rate()).powf(self.compounding_periods())
    }

    /// Yield to maturity of the bond
    pub fn yield_to_maturity(&self) -> f64 {
        ((self.par_value / self.present_value()) - 1.0).powf(1. / self.compounding_periods())
    }

    /// Calculate the present value of the bond
    pub fn present_value(&self) -> f64 {
        self.par_value / (1.0 + self.periodic_rate()).powf(self.compounding_periods())
    }

    /// Return the annual cash flow from the coupon payment
    pub fn annual_cash_flow(&self) -> f64 {
        self.coupon_payment() * self.frequency.to_f64()
    }

    /// Calculate the current yield of the bond
    pub fn current_yield(&self) -> f64 {
        self.annual_cash_flow() / self.present_value()
    }

    /// Calculate the Macaulay duration of the bond
    pub fn duration(&self, market_price: Option<f64>) -> f64 {
        let mut duration = 0.0;
        let mut t = 1.0;
        let coupon_payment = self.coupon_payment();
        // let future_value = self.future_value();
        let price = market_price.unwrap_or(self.present_value());
        // let frequency = self.frequency.to_f64();
        let n = self.compounding_periods();

        while t <= n {
            duration += (t * coupon_payment) / (1.0 + self.periodic_rate()).powf(t);
            t += 1.0;
        }

        duration += (n * (self.par_value + coupon_payment)) / (1.0 + self.periodic_rate()).powf(n);

        duration / price
    }

    /// Calculate the modified duration of the bond
    pub fn modified_duration(&self, market_price: Option<f64>) -> f64 {
        self.duration(market_price) / (1.0 + self.yield_to_maturity() / self.compounding_periods())
    }

    /// Return a vector of present-value discounted cash flows for the bond
    pub fn discount_cash_flows(&self) -> Vec<f64> {
        let mut cash_flows = Vec::new();
        let mut t = 1.0;
        let coupon_payment = self.coupon_payment();
        let n = self.compounding_periods();

        while t <= n {
            cash_flows.push(coupon_payment / (1.0 + self.periodic_rate()).powf(t));
            t += 1.0;
        }

        cash_flows.push((self.par_value + coupon_payment) / (1.0 + self.periodic_rate()).powf(n));

        cash_flows
    }

    /// Return a vector cash flows for the bond
    pub fn cash_flows(&self) -> Vec<f64> {
        let mut cash_flows = Vec::new();
        let mut t = 1.0;
        let coupon_payment = self.coupon_payment();
        let n = self.compounding_periods();

        while t <= n {
            cash_flows.push(coupon_payment);
            t += 1.0;
        }

        cash_flows.push(self.par_value + coupon_payment);

        cash_flows
    }
}
