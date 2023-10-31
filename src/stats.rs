pub trait Statistics {
    /// Return a set of floating point 64-bit values
    fn values(&self) -> Vec<f64>;

    /// Return the count of values
    fn count(&self) -> usize {
        self.values().len()
    }

    /// Calculate the population mean
    fn population_mean(&self) -> f64 {
        self.values().iter().sum::<f64>() / self.count() as f64
    }

    /// Calculate the sample mean
    fn sample_mean(&self) -> f64 {
        self.values().iter().sum::<f64>() / (self.count() - 1) as f64
    }

    /// Calculate the geometric mean
    fn geometric_mean(&self) -> f64 {
        self.values()
            .iter()
            .map(|v| 1.0 + v)
            .product::<f64>()
            .powf(1.0 / self.count() as f64)
            - 1.
    }

    /// Calculate the weighted average
    fn weighted_average(&self, weights: Vec<f64>) -> f64 {
        if self.count() != weights.len() {
            return 0.0;
        }

        if weights.iter().sum::<f64>() != 1.0 {
            return 0.0;
        }

        self.values()
            .iter()
            .zip(weights.iter())
            .map(|(v, w)| v * w)
            .sum::<f64>()
    }

    /// Calculate the harmonic mean
    fn harmonic_mean(&self) -> f64 {
        self.values().iter().map(|v| 1.0 / v).sum::<f64>().recip() / self.count() as f64
    }

    /// Calculate the percentile of a distribution
    fn percentile(&self, target_percentile: f64) -> f64 {
        let mut values = self.values();
        // Sorted our values in ascending order
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let index = (self.count() + 1) as f64 * (target_percentile / 100.0);

        if index.fract() == 0.0 {
            return values[index as usize];
        } else {
            let index_floor = index.floor();
            let index_ceil = index.ceil();

            let value_floor = values[index_floor as usize];
            let value_ceil = values[index_ceil as usize];

            let value = value_floor + (index - index_floor) * (value_ceil - value_floor);

            return value;
        }
    }

    /// Calculate the Range of the distribution
    fn range(&self) -> f64 {
        self.max() - self.min()
    }

    fn max(&self) -> f64 {
        let mut value = 0.0;

        for v in self.values() {
            if v > value {
                value = v;
            }
        }

        value
    }

    fn min(&self) -> f64 {
        let mut value = 0.0;

        for v in self.values() {
            if v < value {
                value = v;
            }
        }

        value
    }

    /// Calculate population variance
    fn population_variance(&self) -> f64 {
        let mean = self.expected_probability();

        self.values()
            .iter()
            .map(|v| (v - mean).powf(2.))
            .sum::<f64>()
            / self.count() as f64
    }

    /// Calculate sample variance
    fn sample_variance(&self) -> f64 {
        let mean = self.expected_probability();

        self.values()
            .iter()
            .map(|v| (v - mean).powf(2.))
            .sum::<f64>()
            / (self.count() - 1) as f64
    }

    /// Calculate population standard deviation
    fn population_std_dev(&self) -> f64 {
        self.population_variance().sqrt()
    }

    /// Calculate sample standard deviation
    fn sample_std_dev(&self) -> f64 {
        self.sample_variance().sqrt()
    }

    /// Calculate downside deviation (semideviation)
    fn downside_deviation(&self) -> f64 {
        let mean = self.expected_probability();

        let variance = self
            .values()
            .iter()
            .filter(|v| **v <= mean)
            .map(|v| (v - mean).powf(2.))
            .sum::<f64>()
            / (self.count() - 1) as f64;

        variance.sqrt()
    }

    /// Calculate Coefficience of Variance
    fn coefficient_variance(&self) -> f64 {
        self.sample_std_dev() / self.expected_probability()
    }

    /// Calculate the Sharpe Ratio
    fn sharpe_ratio(&self, risk_free_rate: f64) -> f64 {
        (self.expected_probability() - risk_free_rate) / self.sample_std_dev()
    }

    /// Normalize the values as z-scores
    fn z_scores(&self) -> Vec<f64> {
        let mean = self.expected_probability();
        let std_dev = self.sample_std_dev();

        self.values()
            .iter()
            .map(|v| v - mean / std_dev)
            .collect::<Vec<f64>>()
    }

    /// Calculate the skewness of the distribution
    fn skewness(&self) -> f64 {
        let mean = self.expected_probability();

        (1 / self.count()) as f64
            * self
                .values()
                .iter()
                .map(|v| (v - mean).powf(3.))
                .sum::<f64>()
            / self.sample_std_dev().powf(3.)
    }

    /// Calculate the Excess Kurtosis
    fn excess_kurtosis(&self) -> f64 {
        let mean = self.expected_probability();

        (1 / self.count()) as f64
            * self
                .values()
                .iter()
                .map(|v| (v - mean).powf(4.))
                .sum::<f64>()
            / self.sample_std_dev().powf(4.)
            // Subject 3 becuase kurtosis of a normal distribution is 3
            - 3.0
    }

    /// Calculate Covariance with another distribution
    fn covariance(&self, other: &Self) -> f64 {
        let self_mean = self.expected_probability();
        let other_mean = other.expected_probability();
        let n = (self.count() - 1) as f64;

        self.values()
            .iter()
            .zip(other.values().iter())
            .map(|(a, b)| (a - self_mean) * (b - other_mean))
            .sum::<f64>()
            / n
    }

    /// Calculate Correlation with another distribution
    fn correlation(&self, other: &Self) -> f64 {
        let covariance = self.covariance(other);
        let self_std_dev = self.sample_std_dev();
        let other_std_dev = other.sample_std_dev();

        covariance / (self_std_dev * other_std_dev)
    }

    fn probability(&self, value: &f64) -> f64 {
        self.values().iter().filter(|r| r == &value).count() as f64 / self.values().len() as f64
    }

    fn probability_bounds(&self, lower_bound: f64, upper_bound: f64) -> f64 {
        self.values()
            .iter()
            .filter(|r| *r >= &lower_bound && *r <= &upper_bound)
            .count() as f64
            / self.values().len() as f64
    }

    /// Weighted-average probability
    fn expected_probability(&self) -> f64 {
        let mut expected_p = 0.0;

        let mantissa = 10_f64.powf(5.);

        for target_return in self.values().iter() {
            let lower_bound = ((target_return * mantissa).floor()) / mantissa;
            let upper_bound = ((target_return * mantissa).ceil()) / mantissa;

            let prob = self.probability_bounds(lower_bound, upper_bound);

            let value = prob * target_return;
            expected_p += value;
        }

        expected_p
    }
}

// pub trait Statistics {
//     /// Return the vector of values
//     fn values(&self) -> Vec<f64>;

//     /// Return the count of values in the vector
//     fn count(&self) -> f64 {
//         self.values().len() as f64
//     }

//     /// Calculate the population mean of a vector of values
//     fn population_mean(&self) -> f64 {
//         self.values().iter().sum::<f64>() / self.count()
//     }

//     /// Calculate the sample mean of a vector of values
//     fn sample_mean(&self) -> f64 {
//         self.values().iter().sum::<f64>() / (self.count() - 1.0)
//     }

//     /// Calculate the geometric mean of a vector of values
//     fn geometric_mean(&self) -> f64 {
//         self.values()
//             .iter()
//             .map(|v| 1. + v)
//             .product::<f64>()
//             .powf(1.0 / self.count())
//             - 1.0
//     }

//     /// Calculate the weighted average mean
//     /// Weights must sum to 1, and must be the same length as the values
//     fn weighted_average_mean(&self, weights: Vec<f64>) -> f64 {
//         if self.count() != weights.len() as f64 {
//             return 0.0;
//         }

//         if weights.iter().sum::<f64>() != 1.0 {
//             return 0.0;
//         }

//         self.values()
//             .iter()
//             .zip(weights.iter())
//             .map(|(v, w)| v * w)
//             .sum::<f64>()
//     }

//     /// Calculate the harmonic mean of a vector of values
//     fn harmonic_mean(&self) -> f64 {
//         self.values().iter().map(|v| 1.0 / v).sum::<f64>().recip() / self.count()
//     }

//     /// Calculate the percentile of a vector of values
//     fn percentile(&self, p: f64) -> f64 {
//         let mut values = self.values();
//         values.sort_by(|a, b| a.partial_cmp(b).unwrap());

//         let rank = (p / 100.0 * (self.count() - 1.0)) - 1.0;

//         if rank.fract() == 0.0 {
//             values[rank as usize]
//         } else {
//             let rank_floor = rank.floor();
//             let rank_ceil = rank.ceil();

//             let value_floor = values[rank_floor as usize];
//             let value_ceil = values[rank_ceil as usize];

//             value_floor + (value_ceil - value_floor) * (rank - rank_floor)
//         }
//     }

//     /// Calculate the range of a vector of values
//     fn range(&self) -> f64 {
//         let mut values = self.values();
//         values.sort_by(|a, b| a.partial_cmp(b).unwrap());

//         values.last().unwrap() - values.first().unwrap()
//     }

//     /// Calculate the population variance of a vector of values
//     fn population_variance(&self) -> f64 {
//         let mean = self.population_mean();
//         self.values()
//             .iter()
//             .map(|v| (v - mean).powf(2.0))
//             .sum::<f64>()
//             / self.count()
//     }

//     /// Calculate the sample variance of a vector of values
//     fn sample_variance(&self) -> f64 {
//         let mean = self.sample_mean();
//         self.values()
//             .iter()
//             .map(|v| (v - mean).powf(2.0))
//             .sum::<f64>()
//             / (self.count() - 1.0)
//     }

//     /// Calculate the population standard deviation of a vector of values
//     fn population_standard_deviation(&self) -> f64 {
//         self.population_variance().sqrt()
//     }

//     /// Calculate the sample standard deviation of a vector of values
//     fn sample_standard_deviation(&self) -> f64 {
//         self.sample_variance().sqrt()
//     }

//     /// Calculate the semideviation (downside) of a vector of values
//     fn semideviation(&self) -> f64 {
//         let mean = self.population_mean();
//         let variance = self
//             .values()
//             .iter()
//             .filter(|v| **v <= mean)
//             .map(|v| (v - mean).powf(2.0))
//             .sum::<f64>()
//             / self.count();
//         variance.sqrt()
//     }

//     /// Calculate the coefficient of variation of a vector of values
//     fn coefficient_of_variation(&self) -> f64 {
//         self.population_standard_deviation() / self.population_mean()
//     }

//     /// Calculate the Sharpe Ratio of a vector of values
//     fn sharpe_ratio(&self, risk_free_rate: f64) -> f64 {
//         (self.population_mean() - risk_free_rate) / self.population_standard_deviation()
//     }

//     /// Calculate the Z-Score (Normal) Scores of a vector of values
//     fn z_scores(&self) -> Vec<f64> {
//         let mean = self.population_mean();
//         let standard_deviation = self.population_standard_deviation();

//         self.values()
//             .iter()
//             .map(|v| (v - mean) / standard_deviation)
//             .collect()
//     }

//     /// Calculate the skewness of a vector of values
//     fn skewness(&self) -> f64 {
//         let mean = self.population_mean();
//         let standard_deviation = self.population_standard_deviation();

//         let n = self.count();

//         let skewness = self
//             .values()
//             .iter()
//             .map(|v| (v - mean) / standard_deviation)
//             .map(|v| v.powf(3.0))
//             .sum::<f64>()
//             / n;

//         skewness
//     }

//     /// Calculate the kurtosis of a vector of values
//     fn excess_kurtosis(&self) -> f64 {
//         let mean = self.population_mean();
//         let standard_deviation = self.population_standard_deviation();

//         let n = self.count();

//         let kurtosis = self
//             .values()
//             .iter()
//             .map(|v| (v - mean) / standard_deviation)
//             .map(|v| v.powf(4.0))
//             .sum::<f64>()
//             / n;

//         kurtosis - 3.0
//     }

//     /// Calculate the covariance of two vectors of values
//     fn covariance(&self, other: &Self) -> f64 {
//         let mean_self = self.population_mean();
//         let mean_other = other.population_mean();

//         let n = self.count();

//         let covariance = self
//             .values()
//             .iter()
//             .zip(other.values().iter())
//             .map(|(v1, v2)| (v1 - mean_self) * (v2 - mean_other))
//             .sum::<f64>()
//             / n;

//         covariance
//     }

//     /// Calculate the correlation of two vectors of values
//     fn correlation(&self, other: &Self) -> f64 {
//         let covariance = self.covariance(other);
//         let standard_deviation_self = self.population_standard_deviation();
//         let standard_deviation_other = other.population_standard_deviation();

//         covariance / (standard_deviation_self * standard_deviation_other)
//     }

//     /// Calculate the probability of a value in a vector of values, given an error tolerance
//     fn probability(&self, value: f64, error_tolerance: f64) -> f64 {
//         let mean = self.population_mean();
//         let standard_deviation = self.population_standard_deviation();

//         let z_score = (value - mean) / standard_deviation;

//         let probability = 1.0 - error_tolerance * z_score;

//         probability
//     }

//     /// Calculate the probability of a value between an upper and lower bound in a vector of values
//     fn probability_between(&self, lower_bound: f64, upper_bound: f64) -> f64 {
//         let mean = self.population_mean();
//         let standard_deviation = self.population_standard_deviation();

//         let z_score_lower = (lower_bound - mean) / standard_deviation;
//         let z_score_upper = (upper_bound - mean) / standard_deviation;

//         let probability_lower = 1.0 - z_score_lower;
//         let probability_upper = 1.0 - z_score_upper;

//         probability_upper - probability_lower
//     }

//     /// Calculate Bayes Theorem
//     fn bayes_theorem(&self, other: &Self) -> f64 {
//         let p_a = self.count() / (self.count() + other.count());
//         let p_b = other.count() / (self.count() + other.count());
//         let p_b_given_a = self.correlation(other);

//         p_b_given_a * p_a / p_b
//     }
// }
