use futures_util::Future;
use serde::Deserialize;

use super::Client;
use crate::http::{HttpError, HttpVerb};

pub enum Interval {
    Minute,
    Hour,
    Day,
}

#[derive(Clone, Debug, Deserialize)]
pub struct VolumeTick {
    pub timestamp: usize,
    pub volumes: Vec<ExchangeVolume>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct VolumeTickResponse {
    pub data: Vec<VolumeTick>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ExchangeVolume {
    pub exchange: String,
    pub volume: f64,
    pub usd_notional: f64,
}

impl Client {
    pub fn volume(
        self,
        start_time: usize,
        end_time: usize,
        interval: Interval,
        currency: &str,
        net: bool,
        by_exchange: bool,
    ) -> impl Future<Output = Result<VolumeTickResponse, HttpError>> {
        let query_str = format!(
            "analytics/volume?start_time={}&end_time={}&interval={}&currency={}&net={}&by_exchange={}",
            start_time, end_time, convert_interval(interval), currency, net, by_exchange
        );
        let url = self.url_for_v1_resource(&query_str);

        self.request(HttpVerb::Get, &url, None)
    }
}

fn convert_interval<'a>(interval: Interval) -> &'a str {
    match interval {
        Interval::Minute => "60",
        Interval::Hour => "3600",
        Interval::Day => "86400",
    }
}
