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
        let gauge = Gauge::new_with_timestamp(name, labels, value, timestamp);

        self.gauges.push(gauge);
    }

    pub fn info(&mut self, name: impl Into<String>, labels: impl Into<String>, timestamp: i64) {
        let info = Info::new(name, labels, timestamp);

        self.infos.push(info);
    }

    pub fn gauges(&self) -> &Vec<Gauge> {
        &self.gauges
    }

    pub fn infos(&self) -> &Vec<Info> {
        &self.infos
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
                "weather_air_quality_pm25_index",
                format!("{station}, period=\"instant\", location=\"outdoor\""),
                aqi_pm25,
                timestamp,
            );
        }

        if let Some(aqi_pm25) = report.aqi_pm25_24h {
            metrics.gauge(
                "weather_air_quality_pm25_index",
                format!("{station}, period=\"24h average\", location=\"outdoor\""),
                aqi_pm25,
                timestamp,
            );
        }

        if let Some(aqi_pm25) = report.aqi_pm25_24h_aqin {
            metrics.gauge(
                "weather_air_quality_pm25_index",
                format!("{station}, period=\"24h average\", location=\"indoor\""),
                aqi_pm25,
                timestamp,
            );
        }

        if let Some(aqi_pm25) = report.aqi_pm25_aqin {
            metrics.gauge(
                "weather_air_quality_pm25_index",
                format!("{station}, period=\"instant\", location=\"indoor\""),
                aqi_pm25,
                timestamp,
            );
        }

        if let Some(barometric_pressure) = report.baromabsin {
            metrics.gauge(
                "weather_barometric_pressure_inches_hg",
                format!("{station}, location=\"indoor\", measurement=\"absolute\""),
                barometric_pressure,
                timestamp,
            );
        }

        if let Some(barometric_pressure) = report.baromrelin {
            metrics.gauge(
                "weather_barometric_pressure_inches_hg",
                format!("{station}, location=\"indoor\", measurement=\"relative\""),
                barometric_pressure,
                timestamp,
            );
        }

        if let Some(battery) = report.batt1 {
            metrics.gauge(
                "weather_battery",
                format!("{station}, device=\"sensor 1\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.batt2 {
            metrics.gauge(
                "weather_battery",
                format!("{station}, device=\"sensor 2\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.batt3 {
            metrics.gauge(
                "weather_battery",
                format!("{station}, device=\"sensor 3\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.batt4 {
            metrics.gauge(
                "weather_battery",
                format!("{station}, device=\"sensor 4\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.batt5 {
            metrics.gauge(
                "weather_battery",
                format!("{station}, device=\"sensor 5\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.batt6 {
            metrics.gauge(
                "weather_battery",
                format!("{station}, device=\"sensor 6\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.batt7 {
            metrics.gauge(
                "weather_battery",
                format!("{station}, device=\"sensor 7\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.batt8 {
            metrics.gauge(
                "weather_battery",
                format!("{station}, device=\"sensor 8\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.batt_25 {
            metrics.gauge(
                "weather_battery",
                format!("{station}, device=\"outdoor PM2.5\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.batt_co2 {
            metrics.gauge(
                "weather_battery",
                format!("{station}, device=\"AQIN\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.batt_lightning {
            metrics.gauge(
                "weather_battery",
                format!("{station}, device=\"lightning\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.battin {
            metrics.gauge(
                "weather_battery",
                format!("{station}, device=\"indoor\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.battout {
            metrics.gauge(
                "weather_battery",
                format!("{station}, device=\"weather station\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.battsm1 {
            metrics.gauge(
                "weather_battery",
                format!("{station}, device=\"soil humidity 1\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.battsm2 {
            metrics.gauge(
                "weather_battery",
                format!("{station}, device=\"soil humidity 2\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.battsm3 {
            metrics.gauge(
                "weather_battery",
                format!("{station}, device=\"soil humidity 3\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.battsm4 {
            metrics.gauge(
                "weather_battery",
                format!("{station}, device=\"soil humidity 4\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.battsm5 {
            metrics.gauge(
                "weather_battery",
                format!("{station}, device=\"soil humidity 5\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.battsm6 {
            metrics.gauge(
                "weather_battery",
                format!("{station}, device=\"soil humidity 6\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.battsm7 {
            metrics.gauge(
                "weather_battery",
                format!("{station}, device=\"soil humidity 7\""),
                battery,
                timestamp,
            );
        }

        if let Some(battery) = report.battsm8 {
            metrics.gauge(
                "weather_battery",
                format!("{station}, device=\"soil humidity 8\""),
                battery,
                timestamp,
            );
        }

        if let Some(co2) = report.co2_in_24h_aqin {
            metrics.gauge(
                "weather_co2_ppm",
                format!("{station}, period=\"24h average\", location=\"indoor\""),
                co2,
                timestamp,
            );
        }

        if let Some(co2) = report.co2_in_aqin {
            metrics.gauge(
                "weather_co2_ppm",
                format!("{station}, period=\"instant\", location=\"indoor\""),
                co2,
                timestamp,
            );
        }

        if let Some(dew_point) = report.dew_point_f() {
            metrics.gauge(
                "weather_dew_point_fahrenheit",
                format!("{station}, location=\"outdoor\""),
                dew_point,
                timestamp,
            );
        }

        if let Some(direction) = report.winddir {
            metrics.gauge(
                "weather_wind_degrees",
                format!("{station}, period=\"instant\""),
                direction,
                timestamp,
            );
        }

        if let Some(direction) = report.winddir_avg10m {
            metrics.gauge(
                "weather_wind_degrees",
                format!("{station}, period=\"10m average\""),
                direction,
                timestamp,
            );
        }

        if let Some(speed) = report.windgustmph {
            metrics.gauge(
                "weather_wind_gust_mph",
                format!("{station}, period=\"instant\""),
                speed,
                timestamp,
            );
        }

        if let Some(speed) = report.maxdailygust {
            metrics.gauge(
                "weather_wind_gust_mph",
                format!("{station}, period=\"max daily\""),
                speed,
                timestamp,
            );
        }

        if let Some(humidity) = report.humidity {
            metrics.gauge(
                "weather_humidity_relative",
                format!("{station}, location=\"outdoor\""),
                humidity / 100.0,
                timestamp,
            );
        }

        if let Some(humidity) = report.humidityin {
            metrics.gauge(
                "weather_humidity_relative",
                format!("{station}, location=\"indoor\""),
                humidity / 100.0,
                timestamp,
            );
        }

        if let Some(humidity) = report.pm_in_humidity_aqin {
            metrics.gauge(
                "weather_humidity_relative",
                format!("{station}, location=\"indoor AQIN\""),
                humidity / 100.0,
                timestamp,
            );
        }

        if let Some(strikes) = report.lightning_day {
            metrics.gauge(
                "weather_lightning_strikes",
                format!("{station}"),
                strikes,
                timestamp,
            );
        }

        if let Some(pm10) = report.pm10_in_24h_aqin {
            metrics.gauge(
                "weather_pm10_concentration",
                format!("{station}, period=\"24h average\", location=\"indoor\""),
                pm10,
                timestamp,
            );
        }

        if let Some(pm10) = report.pm10_in_aqin {
            metrics.gauge(
                "weather_pm10_concentration",
                format!("{station}, period=\"instant\", location=\"indoor\""),
                pm10,
                timestamp,
            );
        }

        if let Some(pm25) = report.pm25 {
            metrics.gauge(
                "weather_pm25_concentration",
                format!("{station}, period=\"instant\", location=\"outdoor\""),
                pm25,
                timestamp,
            );
        }

        if let Some(pm25) = report.pm25_24h {
            metrics.gauge(
                "weather_pm25_concentration",
                format!("{station}, period=\"24h average\", location=\"outdoor\""),
                pm25,
                timestamp,
            );
        }

        if let Some(pm25) = report.pm25_in_24h_aqin {
            metrics.gauge(
                "weather_pm25_concentration",
                format!("{station}, period=\"24h average\", location=\"indoor\""),
                pm25,
                timestamp,
            );
        }

        if let Some(pm25) = report.pm25_in_aqin {
            metrics.gauge(
                "weather_pm25_concentration",
                format!("{station}, period=\"instant\", location=\"indoor\""),
                pm25,
                timestamp,
            );
        }

        if let Some(rain) = report.dailyrainin {
            metrics.gauge(
                "weather_rain_inches",
                format!("{station}, period=\"day\""),
                rain,
                timestamp,
            );
        }

        if let Some(rain) = report.eventrainin {
            metrics.gauge(
                "weather_rain_inches",
                format!("{station}, period=\"event\""),
                rain,
                timestamp,
            );
        }

        if let Some(rain) = report.hourlyrainin {
            metrics.gauge(
                "weather_rain_inches",
                format!("{station}, period=\"hour\""),
                rain,
                timestamp,
            );
        }

        if let Some(rain) = report.monthlyrainin {
            metrics.gauge(
                "weather_rain_inches",
                format!("{station}, period=\"month\""),
                rain,
                timestamp,
            );
        }

        if let Some(rain) = report.weeklyrainin {
            metrics.gauge(
                "weather_rain_inches",
                format!("{station}, period=\"week\""),
                rain,
                timestamp,
            );
        }

        if let Some(rain) = report.yearlyrainin {
            metrics.gauge(
                "weather_rain_inches",
                format!("{station}, period=\"year\""),
                rain,
                timestamp,
            );
        }

        if let Some(humidity) = report.soilhum1 {
            metrics.gauge(
                "weather_soil_humidity",
                format!("{station}, device=\"soil humidity 1\""),
                humidity / 100.0,
                timestamp,
            );
        }

        if let Some(humidity) = report.soilhum2 {
            metrics.gauge(
                "weather_soil_humidity",
                format!("{station}, device=\"soil humidity 2\""),
                humidity / 100.0,
                timestamp,
            );
        }

        if let Some(humidity) = report.soilhum3 {
            metrics.gauge(
                "weather_soil_humidity",
                format!("{station}, device=\"soil humidity 3\""),
                humidity / 100.0,
                timestamp,
            );
        }

        if let Some(humidity) = report.soilhum4 {
            metrics.gauge(
                "weather_soil_humidity",
                format!("{station}, device=\"soil humidity 4\""),
                humidity / 100.0,
                timestamp,
            );
        }

        if let Some(humidity) = report.soilhum5 {
            metrics.gauge(
                "weather_soil_humidity",
                format!("{station}, device=\"soil humidity 5\""),
                humidity / 100.0,
                timestamp,
            );
        }

        if let Some(humidity) = report.soilhum6 {
            metrics.gauge(
                "weather_soil_humidity",
                format!("{station}, device=\"soil humidity 6\""),
                humidity / 100.0,
                timestamp,
            );
        }

        if let Some(humidity) = report.soilhum7 {
            metrics.gauge(
                "weather_soil_humidity",
                format!("{station}, device=\"soil humidity 7\""),
                humidity / 100.0,
                timestamp,
            );
        }

        if let Some(humidity) = report.soilhum8 {
            metrics.gauge(
                "weather_soil_humidity",
                format!("{station}, device=\"soil humidity 8\""),
                humidity / 100.0,
                timestamp,
            );
        }

        if let Some(solar_radiation) = report.solarradiation {
            metrics.gauge(
                "weather_solar_radiation",
                format!("{station}"),
                solar_radiation,
                timestamp,
            );
        }

        if let Some(speed) = report.windspeedmph {
            metrics.gauge(
                "weather_wind_mph",
                format!("{station}, period=\"instant\""),
                speed,
                timestamp,
            );
        }

        if let Some(speed) = report.windspdmph_avg10m {
            metrics.gauge(
                "weather_wind_mph",
                format!("{station}, period=\"10m average\""),
                speed,
                timestamp,
            );
        }

        if let Some(temperature) = report.pm_in_temp_aqin {
            metrics.gauge(
                "weather_temperature_fahrenheit",
                format!("{station}, location=\"indoor AQIN\""),
                temperature,
                timestamp,
            );
        }

        if let Some(temperature) = report.tempf {
            metrics.gauge(
                "weather_temperature_fahrenheit",
                format!("{station}, location=\"outdoor\""),
                temperature,
                timestamp,
            );
        }

        if let Some(temperature) = report.tempinf {
            metrics.gauge(
                "weather_temperature_fahrenheit",
                format!("{station}, location=\"indoor\""),
                temperature,
                timestamp,
            );
        }

        if let Some(uv) = report.uv {
            metrics.gauge("weather_uv_index", format!("{station}"), uv, timestamp);
        }

        metrics
    }
}
