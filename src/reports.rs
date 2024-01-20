use crate::{report::Report, Descriptor, Metrics};
use std::{collections::VecDeque, sync::RwLock};

/// Minimum interval for Ambient weather devices is 16 seconds
///
/// This holds about 5 minutes of reports with the minimum interval
const MAX_REPORTS: usize = 20;

pub struct Reports {
    reports: RwLock<VecDeque<Metrics>>,
    descriptors: Vec<Descriptor>,
}

impl Reports {
    pub fn new() -> Self {
        let descriptors = vec![
            Descriptor::info("weather_station_info", "Weather station information"),
            Descriptor::gauge(
                "air_quality_pm25_index",
                "PM2.5 air quality index",
                "air quality index",
            ),
            Descriptor::gauge(
                "air_quality_pm25_average_24h_index",
                "24 hour average PM2.5 air quality index",
                "air quality index",
            ),
            Descriptor::gauge(
                "barometric_pressure_inches_hg",
                "Barometric pressure",
                "inches Hg",
            ),
            Descriptor::gauge("battery", "Battery state", "boolean"),
            Descriptor::gauge("co2_average_24h_ppm", "24 hour average CO₂", "ppm"),
            Descriptor::gauge("co2_ppm", "CO₂ concentration", "ppm"),
            Descriptor::gauge("wind_degrees", "Wind direction", "degrees"),
            Descriptor::gauge(
                "wind_average_10m_degrees",
                "10 minute average wind direction",
                "degrees",
            ),
            Descriptor::gauge("wind_gust_mph", "Wind gust", "mph"),
            Descriptor::gauge("humidity_relative", "Relative humidity", "percent"),
            Descriptor::gauge("lightning_strikes", "Count of lightning strikes", "count"),
            Descriptor::gauge(
                "pm10_average_24h_concentration",
                "24 hour average PM1.0 concentration",
                "micrograms per cubic meter",
            ),
            Descriptor::gauge(
                "pm10_concentration",
                "PM1.0 concentration",
                "micrograms per cubic meter",
            ),
            Descriptor::gauge(
                "pm25_average_24h_concentration",
                "24 hour average PM2.5 concentration",
                "micrograms per cubic meter",
            ),
            Descriptor::gauge(
                "pm25_concentration",
                "PM2.5 concentration",
                "micrograms per cubic meter",
            ),
            Descriptor::gauge("rain_inches", "Rain accumulation", "inch"),
            Descriptor::gauge("soil_humidity", "Relative soil humidity", "percent"),
            Descriptor::gauge(
                "solar_radiation",
                "Solar radiation",
                "watts per square meter",
            ),
            Descriptor::gauge("wind_mph", "Wind speed", "mph"),
            Descriptor::gauge(
                "wind_average_10m_mph",
                "10 minute average wind speed",
                "mph",
            ),
            Descriptor::gauge(
                "temperature_fahrenheit",
                "Temperature in ℉",
                "degrees fahrenheit",
            ),
            Descriptor::gauge("uv_index", "UV index", "UV index"),
        ];

        Self {
            reports: RwLock::default(),
            descriptors,
        }
    }

    pub fn add_report(&self, report: Report) {
        let metrics = report.into();

        let mut reports = self.reports.write().unwrap();

        reports.push_back(metrics);

        if reports.len() > MAX_REPORTS {
            reports.pop_front();
        }
    }

    pub fn metrics(&self) -> String {
        let reports = self.reports.read().unwrap();

        reports
            .iter()
            .map(|report| report.into())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
