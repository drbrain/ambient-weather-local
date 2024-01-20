use crate::{Gauge, Info, Report};

#[derive(Debug, Default)]
pub struct Metrics {
    gauges: Vec<Gauge>,
    infos: Vec<Info>,
}

impl Metrics {
    pub fn gauge(
        &mut self,
        name: impl Into<String>,
        labels: impl Into<String>,
        value: f64,
        timestamp: i64,
    ) {
        let gauge = Gauge::new(name, labels, value, timestamp);

        self.gauges.push(gauge);
    }

    pub fn info(&mut self, name: impl Into<String>, labels: impl Into<String>, timestamp: i64) {
        let info = Info::new(name, labels, timestamp);

        self.infos.push(info);
    }
}

impl From<Report> for Metrics {
    fn from(report: Report) -> Self {
        (&report).into()
    }
}

impl From<&Report> for Metrics {
    fn from(report: &Report) -> Self {
        let mut metrics = Metrics::default();

        let timestamp = report.time.unix_timestamp() * 1000;
        let station = format!("station=\"{}\"", report.passkey);

        metrics.info(
            "weather_station_info",
            format!("{station}, type=\"{}\"", report.station_type),
            timestamp,
        );

        if let Some(aqi_pm25) = report.aqi_pm25 {
            metrics.gauge(
                "air_quality_pm25_index",
                format!("{station}, sensor=\"indoor PM2.5\""),
                aqi_pm25,
                timestamp,
            );
        }

        if let Some(aqi_pm25) = report.aqi_pm25_24h {
            metrics.gauge(
                "air_quality_pm25_average_24h_index",
                format!("{station}, sensor=\"indoor PM2.5\""),
                aqi_pm25,
                timestamp,
            );
        }

        if let Some(aqi_pm25) = report.aqi_pm25_24h_aqin {
            metrics.gauge(
                "air_quality_pm25_average_24h_index",
                format!("{station}, sensor=\"indoor PM2.5\""),
                aqi_pm25,
                timestamp,
            );
        }

        if let Some(aqi_pm25) = report.aqi_pm25_aqin {
            metrics.gauge(
                "air_quality_pm25_index",
                format!("{station}, sensor=\"indoor PM2.5\""),
                aqi_pm25,
                timestamp,
            );
        }

        if let Some(barometric_pressure) = report.baromabsin {
            metrics.gauge(
                "barometric_pressure_inches_hg",
                format!("{station}, sensor=\"indoor\", measurement=\"absolute\""),
                barometric_pressure,
                timestamp,
            );
        }

        if let Some(barometric_pressure) = report.baromrelin {
            metrics.gauge(
                "barometric_pressure_inches_hg",
                format!("{station}, sensor=\"indoor\", measurement=\"relative\""),
                barometric_pressure,
                timestamp,
            );
        }

        if let Some(battery) = report.batt1 {
            metrics.gauge(
                "battery",
                format!("{station}, sensor=\"sensor 1\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.batt2 {
            metrics.gauge(
                "battery",
                format!("{station}, sensor=\"sensor 2\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.batt3 {
            metrics.gauge(
                "battery",
                format!("{station}, sensor=\"sensor 3\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.batt4 {
            metrics.gauge(
                "battery",
                format!("{station}, sensor=\"sensor 4\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.batt5 {
            metrics.gauge(
                "battery",
                format!("{station}, sensor=\"sensor 5\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.batt6 {
            metrics.gauge(
                "battery",
                format!("{station}, sensor=\"sensor 6\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.batt7 {
            metrics.gauge(
                "battery",
                format!("{station}, sensor=\"sensor 7\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.batt8 {
            metrics.gauge(
                "battery",
                format!("{station}, sensor=\"sensor 8\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.batt_25 {
            metrics.gauge(
                "battery",
                format!("{station}, sensor=\"outdoor PM2.5\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.batt_co2 {
            metrics.gauge(
                "battery",
                format!("{station}, sensor=\"indoor PM2.5\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.batt_lightning {
            metrics.gauge(
                "battery",
                format!("{station}, sensor=\"lightning\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.battin {
            metrics.gauge(
                "battery",
                format!("{station}, sensor=\"indoor\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.battout {
            metrics.gauge(
                "battery",
                format!("{station}, sensor=\"weather station\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.battsm1 {
            metrics.gauge(
                "battery",
                format!("{station}, sensor=\"soil humidity 1\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.battsm2 {
            metrics.gauge(
                "battery",
                format!("{station}, sensor=\"soil humidity 2\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.battsm3 {
            metrics.gauge(
                "battery",
                format!("{station}, sensor=\"soil humidity 3\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.battsm4 {
            metrics.gauge(
                "battery",
                format!("{station}, sensor=\"soil humidity 4\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.battsm5 {
            metrics.gauge(
                "battery",
                format!("{station}, sensor=\"soil humidity 5\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.battsm6 {
            metrics.gauge(
                "battery",
                format!("{station}, sensor=\"soil humidity 6\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.battsm7 {
            metrics.gauge(
                "battery",
                format!("{station}, sensor=\"soil humidity 7\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.battsm8 {
            metrics.gauge(
                "battery",
                format!("{station}, sensor=\"soil humidity 8\""),
                battery,
                timestamp,
            );
        }

        if let Some(co2) = report.co2_in_24h_aqin {
            metrics.gauge(
                "co2_average_24h_ppm",
                format!("{station}, sensor=\"indoor PM2.5\""),
                co2,
                timestamp,
            );
        }

        if let Some(co2) = report.co2_in_aqin {
            metrics.gauge(
                "co2_ppm",
                format!("{station}, sensor=\"indoor PM2.5\""),
                co2,
                timestamp,
            );
        }

        if let Some(direction) = report.winddir {
            metrics.gauge(
                "wind_degrees",
                format!("{station}, sensor=\"weather station\""),
                direction,
                timestamp,
            );
        }

        if let Some(direction) = report.winddir_avg10m {
            metrics.gauge(
                "wind_average_10m_degrees",
                format!("{station}, sensor=\"weather station\""),
                direction,
                timestamp,
            );
        }

        if let Some(speed) = report.windgustmph {
            metrics.gauge(
                "wind_gust_mph",
                format!("{station}, sensor=\"weather station\", period=\"instant\""),
                speed,
                timestamp,
            );
        }

        if let Some(speed) = report.maxdailygust {
            metrics.gauge(
                "wind_gust_mph",
                format!("{station}, sensor=\"weather station\", period=\"max daily\""),
                speed,
                timestamp,
            );
        }

        if let Some(humidity) = report.humidity {
            metrics.gauge(
                "humidity_relative",
                format!("{station}, sensor=\"weather station\""),
                humidity / 100.0,
                timestamp,
            );
        }

        if let Some(humidity) = report.humidityin {
            metrics.gauge(
                "humidity_relative",
                format!("{station}, sensor=\"indoor\""),
                humidity / 100.0,
                timestamp,
            );
        }

        if let Some(humidity) = report.pm_in_humidity_aqin {
            metrics.gauge(
                "humidity_relative",
                format!("{station}, sensor=\"indoor PM2.5\""),
                humidity / 100.0,
                timestamp,
            );
        }

        if let Some(strikes) = report.lightning_day {
            metrics.gauge(
                "lightning_strikes",
                format!("{station}, sensor=\"lightning\""),
                strikes,
                timestamp,
            );
        }

        if let Some(pm10) = report.pm10_in_24h_aqin {
            metrics.gauge(
                "pm10_average_24h_concentration",
                format!("{station}, sensor=\"indoor PM2.5\""),
                pm10,
                timestamp,
            );
        }

        if let Some(pm10) = report.pm10_in_aqin {
            metrics.gauge(
                "pm10_concentration",
                format!("{station}, sensor=\"indoor PM2.5\""),
                pm10,
                timestamp,
            );
        }

        if let Some(pm25) = report.pm25 {
            metrics.gauge(
                "pm25_concentration",
                format!("{station}, sensor=\"outdoor PM2.5\""),
                pm25,
                timestamp,
            );
        }

        if let Some(pm25) = report.pm25_24h {
            metrics.gauge(
                "pm25_average_24h_concentration",
                format!("{station}, sensor=\"outdoor PM2.5\""),
                pm25,
                timestamp,
            );
        }

        if let Some(pm25) = report.pm25_in_24h_aqin {
            metrics.gauge(
                "pm25_average_24h_concentration",
                format!("{station}, sensor=\"indoor PM2.5\""),
                pm25,
                timestamp,
            );
        }

        if let Some(pm25) = report.pm25_in_aqin {
            metrics.gauge(
                "pm25_concentration",
                format!("{station}, sensor=\"indoor PM2.5\""),
                pm25,
                timestamp,
            );
        }

        if let Some(rain) = report.dailyrainin {
            metrics.gauge(
                "rain_inches",
                format!("{station}, sensor=\"rain\", period=\"day\""),
                rain,
                timestamp,
            );
        }

        if let Some(rain) = report.eventrainin {
            metrics.gauge(
                "rain_inches",
                format!("{station}, sensor=\"rain\", period=\"event\""),
                rain,
                timestamp,
            );
        }

        if let Some(rain) = report.hourlyrainin {
            metrics.gauge(
                "rain_inches",
                format!("{station}, sensor=\"rain\", period=\"hour\""),
                rain,
                timestamp,
            );
        }

        if let Some(rain) = report.monthlyrainin {
            metrics.gauge(
                "rain_inches",
                format!("{station}, sensor=\"rain\", period=\"month\""),
                rain,
                timestamp,
            );
        }

        if let Some(rain) = report.weeklyrainin {
            metrics.gauge(
                "rain_inches",
                format!("{station}, sensor=\"rain\", period=\"week\""),
                rain,
                timestamp,
            );
        }

        if let Some(rain) = report.yearlyrainin {
            metrics.gauge(
                "rain_inches",
                format!("{station}, sensor=\"rain\", period=\"year\""),
                rain,
                timestamp,
            );
        }

        if let Some(humidity) = report.soilhum1 {
            metrics.gauge(
                "soil_humidity",
                format!("{station}, sensor=\"soil humidity 1\""),
                humidity / 100.0,
                timestamp,
            );
        }

        if let Some(humidity) = report.soilhum2 {
            metrics.gauge(
                "soil_humidity",
                format!("{station}, sensor=\"soil humidity 2\""),
                humidity / 100.0,
                timestamp,
            );
        }

        if let Some(humidity) = report.soilhum3 {
            metrics.gauge(
                "soil_humidity",
                format!("{station}, sensor=\"soil humidity 3\""),
                humidity / 100.0,
                timestamp,
            );
        }

        if let Some(humidity) = report.soilhum4 {
            metrics.gauge(
                "soil_humidity",
                format!("{station}, sensor=\"soil humidity 4\""),
                humidity / 100.0,
                timestamp,
            );
        }

        if let Some(humidity) = report.soilhum5 {
            metrics.gauge(
                "soil_humidity",
                format!("{station}, sensor=\"soil humidity 5\""),
                humidity / 100.0,
                timestamp,
            );
        }

        if let Some(humidity) = report.soilhum6 {
            metrics.gauge(
                "soil_humidity",
                format!("{station}, sensor=\"soil humidity 6\""),
                humidity / 100.0,
                timestamp,
            );
        }

        if let Some(humidity) = report.soilhum7 {
            metrics.gauge(
                "soil_humidity",
                format!("{station}, sensor=\"soil humidity 7\""),
                humidity / 100.0,
                timestamp,
            );
        }

        if let Some(humidity) = report.soilhum8 {
            metrics.gauge(
                "soil_humidity",
                format!("{station}, sensor=\"soil humidity 8\""),
                humidity / 100.0,
                timestamp,
            );
        }

        if let Some(solar_radiation) = report.solarradiation {
            metrics.gauge(
                "solar_radiation",
                format!("{station}, sensor=\"weather station\""),
                solar_radiation,
                timestamp,
            );
        }

        if let Some(speed) = report.windspeedmph {
            metrics.gauge(
                "wind_mph",
                format!("{station}, sensor=\"weather station\""),
                speed,
                timestamp,
            );
        }

        if let Some(speed) = report.windspdmph_avg10m {
            metrics.gauge(
                "wind_average_10m_mph",
                format!("{station}, sensor=\"weather station\""),
                speed,
                timestamp,
            );
        }

        if let Some(temperature) = report.pm_in_temp_aqin {
            metrics.gauge(
                "temperature_fahrenheit",
                format!("{station}, sensor=\"indoor PM2.5\""),
                temperature,
                timestamp,
            );
        }

        if let Some(temperature) = report.tempf {
            metrics.gauge(
                "temperature_fahrenheit",
                format!("{station}, sensor=\"weather station\""),
                temperature,
                timestamp,
            );
        }

        if let Some(temperature) = report.tempinf {
            metrics.gauge(
                "temperature_fahrenheit",
                format!("{station}, sensor=\"indoor\""),
                temperature,
                timestamp,
            );
        }

        if let Some(uv) = report.uv {
            metrics.gauge(
                "uv_index",
                format!("{station}, sensor=\"weather station\""),
                uv,
                timestamp,
            );
        }

        metrics
    }
}

impl Into<String> for Metrics {
    fn into(self) -> String {
        (&self).into()
    }
}

impl Into<String> for &Metrics {
    fn into(self) -> String {
        let infos = self
            .infos
            .iter()
            .map(|info| info.into())
            .collect::<Vec<String>>()
            .join("\n");
        let gauges = self
            .gauges
            .iter()
            .map(|info| info.into())
            .collect::<Vec<String>>()
            .join("\n");

        format!("{infos}\n{gauges}")
    }
}
