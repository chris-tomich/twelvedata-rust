use derive_builder::Builder;

#[derive(Clone, Copy)]
pub enum Interval {
    Minutes1,
    Minutes5,
    Minutes15,
    Minutes30,
    Minutes45,
    Hours1,
    Hours2,
    Hours4,
    Days1,
    Weeks1,
    Months1,
}

impl Interval {
    pub fn as_str(&self) -> &'static str {
        match self {
            Interval::Minutes1 => "1min",
            Interval::Minutes5 => "5min",
            Interval::Minutes15 => "15min",
            Interval::Minutes30 => "30min",
            Interval::Minutes45 => "45min",
            Interval::Hours1 => "1h",
            Interval::Hours2 => "2h",
            Interval::Hours4 => "4h",
            Interval::Days1 => "1day",
            Interval::Weeks1 => "1week",
            Interval::Months1 => "1month",
        }
    }
}

#[derive(Clone, Copy)]
pub enum InstrumentType {
    Stock,
    Index,
    Etf,
    Reit,
}

impl InstrumentType {
    pub fn as_str(&self) -> &'static str {
        match self {
            InstrumentType::Stock => "Stock",
            InstrumentType::Index => "Index",
            InstrumentType::Etf => "ETF",
            InstrumentType::Reit => "REIT",
        }
    }
}

#[derive(Clone, Copy)]
pub enum ResponseDataFormat {
    Csv,
    Json,
}

impl ResponseDataFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResponseDataFormat::Csv => "CSV",
            ResponseDataFormat::Json => "JSON",
        }
    }
}

#[derive(Clone, Copy)]
pub enum ExchangeType {
    Stock,
    Index,
    Etf,
}

#[allow(dead_code)]
#[derive(Builder, Default)]
pub struct ExchangesParams {
    #[builder(setter(strip_option))]
    exchange_type: Option<ExchangeType>,
}

#[allow(dead_code)]
#[derive(Builder, Default)]
pub struct TimeSeriesParams<'a> {
    /// Exchange where instrument is traded.
    #[builder(setter(strip_option))]
    exchange: Option<&'a str>,
    /// Country where instrument is traded.
    #[builder(setter(strip_option))]
    country: Option<&'a str>,
    /// Type to which instrument belongs.
    #[builder(setter(strip_option))]
    instrument_type: Option<InstrumentType>,
    /// Number of data points to retrieve between 1 to 5000. Server defaults to 30 if none provided.
    #[builder(setter(strip_option))]
    output_size: Option<u16>,
    /// Format of the response data. Server defaults to JSON if none provided.
    #[builder(setter(strip_option))]
    format: Option<ResponseDataFormat>,
}

impl<'a> TimeSeriesParams<'a> {
    pub fn to_string(&self) -> String {
        let mut query = "".to_owned();

        if let Some(exchange) = self.exchange {
            query.push_str("&exchange=");
            query.push_str(exchange);
        }

        if let Some(country) = self.country {
            query.push_str("&country=");
            query.push_str(country);
        }

        if let Some(instrument_type) = self.instrument_type {
            query.push_str("&type=");
            query.push_str(instrument_type.as_str());
        }

        if let Some(output_size) = self.output_size {
            query.push_str("&outputsize=");
            query.push_str(&output_size.to_string());
        }

        if let Some(format) = self.format {
            query.push_str("&format=");
            query.push_str(format.as_str());
        }

        query
    }
}

const URL_PREFIX: &'static str = "https://api.twelvedata.com/";

pub struct TDRequestBuilder {
    apikey: &'static str,
}

impl TDRequestBuilder {
    pub fn new(apikey: &'static str) -> TDRequestBuilder {
        TDRequestBuilder {
            apikey,
        }
    }

    pub fn exchanges(&self) -> String {
        format!("/exchanges")
    }

    pub fn time_series<'a>(&self, symbol: &'a str, interval: Interval, params: &TimeSeriesParams<'a>) -> String {
        format!("{}time_series?symbol={}&interval={}&apikey={}{}", URL_PREFIX, symbol, interval.as_str(), self.apikey, params.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Interval, TDRequestBuilder, TimeSeriesParams};

    #[test]
    fn time_series_test() {
        let params_builder = TimeSeriesParams::default();
        let request_builder = TDRequestBuilder::new("test_api_key");
        let request_uri = request_builder.time_series("AAPL", Interval::Minutes1, &params_builder);

        assert_eq!("https://api.twelvedata.com/time_series?symbol=AAPL&interval=1min&apikey=test_api_key", request_uri);
    }
}
