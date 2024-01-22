use crate::{report::Report, Descriptor, Encoder, Gauge, Info, Metrics};
use std::{
    collections::{HashMap, VecDeque},
    sync::RwLock,
    time::Instant,
};

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
                "barometric_pressure_inches_hg",
                "Barometric pressure",
                "inches Hg",
            ),
            Descriptor::gauge("battery", "Battery state", "boolean"),
            Descriptor::gauge("co2_ppm", "CO₂ concentration", "ppm"),
            Descriptor::gauge("wind_degrees", "Wind direction", "degrees"),
            Descriptor::gauge("wind_gust_mph", "Wind gust", "mph"),
            Descriptor::gauge("humidity_relative", "Relative humidity", "percent"),
            Descriptor::gauge("lightning_strikes", "Count of lightning strikes", "count"),
            Descriptor::gauge(
                "pm10_concentration",
                "PM1.0 concentration",
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

    pub fn encode(&self) -> Result<String, std::fmt::Error> {
        let start = Instant::now();

        let mut buf = String::new();
        let mut encoder = Encoder::new(&mut buf);

        let mut gauge_map: HashMap<String, Vec<Gauge>> = HashMap::default();
        let mut info_map: HashMap<String, Vec<Info>> = HashMap::default();

        for report in self.reports.read().unwrap().iter() {
            for gauge in report.gauges().iter() {
                let list = gauge_map.entry(gauge.metric().name().into()).or_default();
                list.push(gauge.clone());
            }

            for info in report.infos().iter() {
                let list = info_map.entry(info.metric().name().into()).or_default();
                list.push(info.clone());
            }
        }

        for descriptor in &self.descriptors {
            encoder.encode_descriptor(descriptor)?;

            if let Some(infos) = info_map.get_mut(&descriptor.name().to_string()) {
                infos.sort_by_cached_key(|info| info.metric().clone());

                for info in infos {
                    encoder.encode_info(info)?;
                }
            };

            if let Some(gauges) = gauge_map.get_mut(&descriptor.name().to_string()) {
                gauges.sort_by_cached_key(|gauge| gauge.metric().clone());

                for gauge in gauges {
                    encoder.encode_gauge(gauge)?;
                }
            };
        }

        let name = "local_ambient_weather_scrape_duration_seconds";
        let scrape_duration_descriptor = Descriptor::gauge(name, "Scrape duration", "seconds");
        encoder.encode_descriptor(&scrape_duration_descriptor)?;

        let scrape_duration = start.elapsed().as_secs_f64();
        let scrape_duration = Gauge::new(name, "", scrape_duration);

        encoder.encode_gauge(&scrape_duration)?;

        Ok(buf)
    }
}
