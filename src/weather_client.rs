extern crate drawille;

use crate::forecast::{json, ForecastWrapper};

pub struct WeatherClient {
    base_url: String,
    lon: f64,
    lat: f64,
}

impl WeatherClient {
    pub fn new(base_url: String, lon: f64, lat: f64) -> WeatherClient {
        WeatherClient { base_url, lon, lat }
    }

    pub fn get_forecast(&self) -> ForecastWrapper {
        let url = format!(
            "{}/api/category/pmp3g/version/2/geotype/point/lon/{:.3}/lat/{:.3}/data.json",
            self.base_url, self.lon, self.lat
        );
        let resp = match reqwest::blocking::get(url) {
            Ok(resp) => resp,
            Err(err) => panic!("Failed to get response {:?}", err),
        };

        let forecast: json::Forecast = resp.json().unwrap();

        ForecastWrapper::new(forecast)
    }
}
