use crate::ambient_weather_report::AmbientWeatherReport;
use time::{
    format_description::FormatItem, macros::format_description, OffsetDateTime, PrimitiveDateTime,
};

#[derive(Debug)]
pub struct Report {
    station_type: String,
    passkey: String,
    time: OffsetDateTime,
    aqi_pm25: Option<f64>,
    aqi_pm25_24h: Option<f64>,
    aqi_pm25_24h_aqin: Option<f64>,
    aqi_pm25_aqin: Option<f64>,
    baromabsin: Option<f64>,
    baromrelin: Option<f64>,
    batt1: Option<f64>,
    batt2: Option<f64>,
    batt3: Option<f64>,
    batt4: Option<f64>,
    batt5: Option<f64>,
    batt6: Option<f64>,
    batt7: Option<f64>,
    batt8: Option<f64>,
    batt_25: Option<f64>,
    batt_co2: Option<f64>,
    batt_lightning: Option<f64>,
    battin: Option<f64>,
    battout: Option<f64>,
    battsm1: Option<f64>,
    battsm2: Option<f64>,
    battsm3: Option<f64>,
    battsm4: Option<f64>,
    battsm5: Option<f64>,
    battsm6: Option<f64>,
    battsm7: Option<f64>,
    battsm8: Option<f64>,
    co2_in_24h_aqin: Option<f64>,
    co2_in_aqin: Option<f64>,
    dailyrainin: Option<f64>,
    eventrainin: Option<f64>,
    hourlyrainin: Option<f64>,
    humidity: Option<f64>,
    humidityin: Option<f64>,
    lightning_day: Option<f64>,
    maxdailygust: Option<f64>,
    monthlyrainin: Option<f64>,
    pm10_in_24h_aqin: Option<f64>,
    pm10_in_aqin: Option<f64>,
    pm25: Option<f64>,
    pm25_24h: Option<f64>,
    pm25_in_24h_aqin: Option<f64>,
    pm25_in_aqin: Option<f64>,
    pm_in_humidity_aqin: Option<f64>,
    pm_in_temp_aqin: Option<f64>,
    soilhum1: Option<f64>,
    soilhum2: Option<f64>,
    soilhum3: Option<f64>,
    soilhum4: Option<f64>,
    soilhum5: Option<f64>,
    soilhum6: Option<f64>,
    soilhum7: Option<f64>,
    soilhum8: Option<f64>,
    solarradiation: Option<f64>,
    tempf: Option<f64>,
    tempinf: Option<f64>,
    uv: Option<f64>,
    weeklyrainin: Option<f64>,
    winddir: Option<f64>,
    winddir_avg10m: Option<f64>,
    windgustmph: Option<f64>,
    windspdmph_avg10m: Option<f64>,
    windspeedmph: Option<f64>,
    yearlyrainin: Option<f64>,
}

impl Into<String> for &Report {
    fn into(self) -> String {
        let timestamp = self.time.unix_timestamp() * 1000;
        let station = format!("station=\"{}\"", self.passkey);

        let mut metrics = vec![];

        metrics.push(format!(
            "weather_station_info{{{station}, type=\"{}\"}} 1 {timestamp}",
            self.station_type
        ));

        if let Some(aqi_pm25) = self.aqi_pm25 {
            metrics.push(format!(
                "air_quality_pm25_index{{{station}, sensor=\"indoor PM2.5\"}} {aqi_pm25} {timestamp}"
            ));
        }

        if let Some(aqi_pm25) = self.aqi_pm25_24h {
            metrics.push(format!(
                "air_quality_pm25_average_24h_index{{{station}, sensor=\"indoor PM2.5\"}} {aqi_pm25} {timestamp}"
            ));
        }

        if let Some(aqi_pm25) = self.aqi_pm25_24h_aqin {
            metrics.push(format!(
                "air_quality_pm25_average_24h_index{{{station}, sensor=\"indoor PM2.5\"}} {aqi_pm25} {timestamp}"
            ));
        }

        if let Some(aqi_pm25) = self.aqi_pm25_aqin {
            metrics.push(format!(
                "air_quality_pm25_index{{{station}, sensor=\"indoor PM2.5\"}} {aqi_pm25} {timestamp}"
            ));
        }

        if let Some(barometric_pressure) = self.baromabsin {
            metrics.push(format!(
                "barometric_pressure_inches_hg{{{station}, sensor=\"indoor\", measurement=\"absolute\"}} {barometric_pressure} {timestamp}"
            ));
        }

        if let Some(barometric_pressure) = self.baromrelin {
            metrics.push(format!(
                "barometric_pressure_inches_hg{{{station}, sensor=\"indoor\", measurement=\"relative\"}} {barometric_pressure} {timestamp}"
            ));
        }

        if let Some(battery) = self.batt1 {
            metrics.push(format!(
                "battery{{{station}, sensor=\"sensor 1\"}} {battery} {timestamp}"
            ));
        }

        if let Some(battery) = self.batt2 {
            metrics.push(format!(
                "battery{{{station}, sensor=\"sensor 2\"}} {battery} {timestamp}"
            ));
        }

        if let Some(battery) = self.batt3 {
            metrics.push(format!(
                "battery{{{station}, sensor=\"sensor 3\"}} {battery} {timestamp}"
            ));
        }

        if let Some(battery) = self.batt4 {
            metrics.push(format!(
                "battery{{{station}, sensor=\"sensor 4\"}} {battery} {timestamp}"
            ));
        }

        if let Some(battery) = self.batt5 {
            metrics.push(format!(
                "battery{{{station}, sensor=\"sensor 5\"}} {battery} {timestamp}"
            ));
        }

        if let Some(battery) = self.batt6 {
            metrics.push(format!(
                "battery{{{station}, sensor=\"sensor 6\"}} {battery} {timestamp}"
            ));
        }

        if let Some(battery) = self.batt7 {
            metrics.push(format!(
                "battery{{{station}, sensor=\"sensor 7\"}} {battery} {timestamp}"
            ));
        }

        if let Some(battery) = self.batt8 {
            metrics.push(format!(
                "battery{{{station}, sensor=\"sensor 8\"}} {battery} {timestamp}"
            ));
        }

        if let Some(battery) = self.batt_25 {
            metrics.push(format!(
                "battery{{{station}, sensor=\"indoor PM2.5\"}} {battery} {timestamp}"
            ));
        }

        if let Some(battery) = self.batt_co2 {
            metrics.push(format!(
                "battery{{{station}, sensor=\"indoor PM2.5\"}} {battery} {timestamp}"
            ));
        }

        if let Some(battery) = self.batt_lightning {
            metrics.push(format!(
                "battery{{{station}, sensor=\"lightning\"}} {battery} {timestamp}"
            ));
        }

        if let Some(battery) = self.battin {
            metrics.push(format!(
                "battery{{{station}, sensor=\"indoor\"}} {battery} {timestamp}"
            ));
        }

        if let Some(battery) = self.battout {
            metrics.push(format!(
                "battery{{{station}, sensor=\"weather station\"}} {battery} {timestamp}"
            ));
        }

        if let Some(battery) = self.battsm1 {
            metrics.push(format!(
                "battery{{{station}, sensor=\"soil humidity 1\"}} {battery} {timestamp}"
            ));
        }

        if let Some(battery) = self.battsm2 {
            metrics.push(format!(
                "battery{{{station}, sensor=\"soil humidity 2\"}} {battery} {timestamp}"
            ));
        }

        if let Some(battery) = self.battsm3 {
            metrics.push(format!(
                "battery{{{station}, sensor=\"soil humidity 3\"}} {battery} {timestamp}"
            ));
        }

        if let Some(battery) = self.battsm4 {
            metrics.push(format!(
                "battery{{{station}, sensor=\"soil humidity 4\"}} {battery} {timestamp}"
            ));
        }

        if let Some(battery) = self.battsm5 {
            metrics.push(format!(
                "battery{{{station}, sensor=\"soil humidity 5\"}} {battery} {timestamp}"
            ));
        }

        if let Some(battery) = self.battsm6 {
            metrics.push(format!(
                "battery{{{station}, sensor=\"soil humidity 6\"}} {battery} {timestamp}"
            ));
        }

        if let Some(battery) = self.battsm7 {
            metrics.push(format!(
                "battery{{{station}, sensor=\"soil humidity 7\"}} {battery} {timestamp}"
            ));
        }

        if let Some(battery) = self.battsm8 {
            metrics.push(format!(
                "battery{{{station}, sensor=\"soil humidity 8\"}} {battery} {timestamp}"
            ));
        }

        if let Some(co2) = self.co2_in_24h_aqin {
            metrics.push(format!(
                "co2_average_24h_ppm{{{station}, sensor=\"indoor PM2.5\"}} {co2} {timestamp}"
            ));
        }

        if let Some(co2) = self.co2_in_aqin {
            metrics.push(format!(
                "co2_ppm{{{station}, sensor=\"indoor PM2.5\"}} {co2} {timestamp}"
            ));
        }

        if let Some(direction) = self.winddir {
            metrics.push(format!(
                "wind_direction_degrees{{{station}, sensor=\"weather station\"}} {direction} {timestamp}"
            ));
        }

        if let Some(direction) = self.winddir_avg10m {
            metrics.push(format!(
                "wind_direction_average_10m_degrees{{{station}, sensor=\"weather station\"}} {direction} {timestamp}"
            ));
        }

        if let Some(gust) = self.windgustmph {
            metrics.push(format!(
                "wind_gust_mph{{{station}, sensor=\"weather station\", period=\"instant\"}} {gust} {timestamp}"
            ));
        }

        if let Some(gust) = self.maxdailygust {
            metrics.push(format!(
                "wind_gust_mph{{{station}, sensor=\"weather station\", period=\"max daily\"}} {gust} {timestamp}"
            ));
        }

        if let Some(humidity) = self.humidity {
            metrics.push(format!(
                "humidity_relative{{{station}, sensor=\"weather station\"}} {humidity} {timestamp}"
            ));
        }

        if let Some(humidity) = self.humidityin {
            metrics.push(format!(
                "humidity_relative{{{station}, sensor=\"indoor\"}} {humidity} {timestamp}"
            ));
        }

        if let Some(humidity) = self.pm_in_humidity_aqin {
            metrics.push(format!(
                "humidity_relative{{{station}, sensor=\"indoor PM2.5\"}} {humidity} {timestamp}"
            ));
        }

        if let Some(strikes) = self.lightning_day {
            metrics.push(format!(
                "lightning_strikes{{{station}, sensor=\"lightning\"}} {strikes} {timestamp}"
            ));
        }

        if let Some(pm10) = self.pm10_in_24h_aqin {
            metrics.push(format!(
                "pm10_average_24h_micrograms_per_cubic_meter{{{station}, sensor=\"indoor PM2.5\"}} {pm10} {timestamp}"
            ));
        }

        if let Some(pm10) = self.pm10_in_aqin {
            metrics.push(format!(
                "pm10_micrograms_per_cubic_meter{{{station}, sensor=\"indoor PM2.5\"}} {pm10} {timestamp}"
            ));
        }

        if let Some(pm25) = self.pm25 {
            metrics.push(format!(
                "pm25_micrograms_per_cubic_meter{{{station}, sensor=\"indoor PM2.5\"}} {pm25} {timestamp}"
            ));
        }

        if let Some(pm25) = self.pm25_24h {
            metrics.push(format!(
                "pm25_average_24h_micrograms_per_cubic_meter{{{station}, sensor=\"indoor PM2.5\"}} {pm25} {timestamp}"
            ));
        }

        if let Some(pm25) = self.pm25_in_24h_aqin {
            metrics.push(format!(
                "pm25_average_24h_micrograms_per_cubic_meter{{{station}, sensor=\"indoor PM2.5\"}} {pm25} {timestamp}"
            ));
        }

        if let Some(pm25) = self.pm25_in_aqin {
            metrics.push(format!(
                "pm25_micrograms_per_cubic_meter{{{station}, sensor=\"indoor PM2.5\"}} {pm25} {timestamp}"
            ));
        }

        if let Some(rain) = self.dailyrainin {
            metrics.push(format!(
                "rain_inches{{{station}, sensor=\"rain\", period=\"day\"}} {rain} {timestamp}"
            ));
        }

        if let Some(rain) = self.eventrainin {
            metrics.push(format!(
                "rain_inches{{{station}, sensor=\"rain\", period=\"event\"}} {rain} {timestamp}"
            ));
        }

        if let Some(rain) = self.hourlyrainin {
            metrics.push(format!(
                "rain_inches{{{station}, sensor=\"rain\", period=\"hour\"}} {rain} {timestamp}"
            ));
        }

        if let Some(rain) = self.monthlyrainin {
            metrics.push(format!(
                "rain_inches{{{station}, sensor=\"rain\", period=\"month\"}} {rain} {timestamp}"
            ));
        }

        if let Some(rain) = self.weeklyrainin {
            metrics.push(format!(
                "rain_inches{{{station}, sensor=\"rain\", period=\"week\"}} {rain} {timestamp}"
            ));
        }

        if let Some(rain) = self.yearlyrainin {
            metrics.push(format!(
                "rain_inches{{{station}, sensor=\"rain\", period=\"year\"}} {rain} {timestamp}"
            ));
        }

        if let Some(humidity) = self.soilhum1 {
            metrics.push(format!(
                "soil_humidity{{{station}, sensor=\"soil humidity 1\"}} {humidity} {timestamp}"
            ));
        }

        if let Some(humidity) = self.soilhum2 {
            metrics.push(format!(
                "soil_humidity{{{station}, sensor=\"soil humidity 2\"}} {humidity} {timestamp}"
            ));
        }

        if let Some(humidity) = self.soilhum3 {
            metrics.push(format!(
                "soil_humidity{{{station}, sensor=\"soil humidity 3\"}} {humidity} {timestamp}"
            ));
        }

        if let Some(humidity) = self.soilhum4 {
            metrics.push(format!(
                "soil_humidity{{{station}, sensor=\"soil humidity 4\"}} {humidity} {timestamp}"
            ));
        }

        if let Some(humidity) = self.soilhum5 {
            metrics.push(format!(
                "soil_humidity{{{station}, sensor=\"soil humidity 5\"}} {humidity} {timestamp}"
            ));
        }

        if let Some(humidity) = self.soilhum6 {
            metrics.push(format!(
                "soil_humidity{{{station}, sensor=\"soil humidity 6\"}} {humidity} {timestamp}"
            ));
        }

        if let Some(humidity) = self.soilhum7 {
            metrics.push(format!(
                "soil_humidity{{{station}, sensor=\"soil humidity 7\"}} {humidity} {timestamp}"
            ));
        }

        if let Some(humidity) = self.soilhum8 {
            metrics.push(format!(
                "soil_humidity{{{station}, sensor=\"soil humidity 8\"}} {humidity} {timestamp}"
            ));
        }

        if let Some(solar_radiation) = self.solarradiation {
            metrics.push(format!(
                "solar_radiation_watts_per_square_meter{{{station}, sensor=\"weather station\"}} {solar_radiation} {timestamp}"
            ));
        }

        if let Some(speed) = self.windspeedmph {
            metrics.push(format!(
                "wind_speed_mph{{{station}, sensor=\"weather station\"}} {speed} {timestamp}"
            ));
        }

        if let Some(speed) = self.windspdmph_avg10m {
            metrics.push(format!(
                "wind_speed_average_10m_mph{{{station}, sensor=\"weather station\"}} {speed} {timestamp}"
            ));
        }

        if let Some(temperature) = self.pm_in_temp_aqin {
            metrics.push(format!(
                "temperature_farhenheit{{{station}, sensor=\"indoor PM2.5\"}} {temperature} {timestamp}"
            ));
        }

        if let Some(temperature) = self.tempf {
            metrics.push(format!(
                "temperature_farhenheit{{{station}, sensor=\"weather station\"}} {temperature} {timestamp}"
            ));
        }

        if let Some(temperature) = self.tempinf {
            metrics.push(format!(
                "temperature_farhenheit{{{station}, sensor=\"indoor\"}} {temperature} {timestamp}"
            ));
        }

        if let Some(uv) = self.uv {
            metrics.push(format!(
                "uv_index{{{station}, sensor=\"weather station\"}} {uv} {timestamp}"
            ));
        }

        metrics.join("\n")
    }
}

impl TryFrom<AmbientWeatherReport> for Report {
    type Error = time::error::Parse;

    fn try_from(report: AmbientWeatherReport) -> Result<Self, Self::Error> {
        (&report).try_into()
    }
}

impl TryFrom<&AmbientWeatherReport> for Report {
    type Error = time::error::Parse;

    fn try_from(report: &AmbientWeatherReport) -> Result<Self, Self::Error> {
        let AmbientWeatherReport {
            station_type,
            passkey,
            dateutc,
            aqi_pm25_24h,
            aqi_pm25_24h_aqin,
            aqi_pm25_aqin,
            baromabsin,
            baromrelin,
            batt1,
            batt2,
            batt3,
            batt4,
            batt5,
            batt6,
            batt7,
            batt8,
            batt_25,
            batt_co2,
            battin,
            battout,
            battsm1,
            battsm2,
            battsm3,
            battsm4,
            battsm5,
            battsm6,
            battsm7,
            battsm8,
            co2_in_24h_aqin,
            co2_in_aqin,
            dailyrainin,
            eventrainin,
            hourlyrainin,
            humidity,
            humidityin,
            maxdailygust,
            monthlyrainin,
            pm10_in_24h_aqin,
            pm10_in_aqin,
            pm25_in_24h_aqin,
            pm25_in_aqin,
            pm_in_humidity_aqin,
            pm_in_temp_aqin,
            soilhum1,
            soilhum2,
            soilhum3,
            soilhum4,
            soilhum5,
            soilhum6,
            soilhum7,
            soilhum8,
            solarradiation,
            tempf,
            tempinf,
            uv,
            weeklyrainin,
            winddir,
            winddir_avg10m,
            windgustmph,
            windspdmph_avg10m,
            windspeedmph,
            yearlyrainin,
            aqi_pm25,
            batt_lightning,
            lightning_day,
            pm25,
            pm25_24h,
        } = report;

        let time = try_into_time(&dateutc)?;

        Ok(Report {
            station_type: station_type.clone(),
            passkey: passkey.clone(),
            time,
            aqi_pm25: *aqi_pm25,
            aqi_pm25_24h: *aqi_pm25_24h,
            aqi_pm25_24h_aqin: *aqi_pm25_24h_aqin,
            aqi_pm25_aqin: *aqi_pm25_aqin,
            baromabsin: *baromabsin,
            baromrelin: *baromrelin,
            batt1: *batt1,
            batt2: *batt2,
            batt3: *batt3,
            batt4: *batt4,
            batt5: *batt5,
            batt6: *batt6,
            batt7: *batt7,
            batt8: *batt8,
            batt_25: *batt_25,
            batt_co2: *batt_co2,
            batt_lightning: *batt_lightning,
            battin: *battin,
            battout: *battout,
            battsm1: *battsm1,
            battsm2: *battsm2,
            battsm3: *battsm3,
            battsm4: *battsm4,
            battsm5: *battsm5,
            battsm6: *battsm6,
            battsm7: *battsm7,
            battsm8: *battsm8,
            co2_in_24h_aqin: *co2_in_24h_aqin,
            co2_in_aqin: *co2_in_aqin,
            dailyrainin: *dailyrainin,
            eventrainin: *eventrainin,
            hourlyrainin: *hourlyrainin,
            humidity: *humidity,
            humidityin: *humidityin,
            lightning_day: *lightning_day,
            maxdailygust: *maxdailygust,
            monthlyrainin: *monthlyrainin,
            pm10_in_aqin: *pm10_in_aqin,
            pm10_in_24h_aqin: *pm10_in_24h_aqin,
            pm25: *pm25,
            pm25_24h: *pm25_24h,
            pm25_in_24h_aqin: *pm25_in_24h_aqin,
            pm25_in_aqin: *pm25_in_aqin,
            pm_in_humidity_aqin: *pm_in_humidity_aqin,
            pm_in_temp_aqin: *pm_in_temp_aqin,
            soilhum1: *soilhum1,
            soilhum2: *soilhum2,
            soilhum3: *soilhum3,
            soilhum4: *soilhum4,
            soilhum5: *soilhum5,
            soilhum6: *soilhum6,
            soilhum7: *soilhum7,
            soilhum8: *soilhum8,
            solarradiation: *solarradiation,
            tempf: *tempf,
            tempinf: *tempinf,
            uv: *uv,
            weeklyrainin: *weeklyrainin,
            winddir: *winddir,
            winddir_avg10m: *winddir_avg10m,
            windgustmph: *windgustmph,
            windspdmph_avg10m: *windspdmph_avg10m,
            windspeedmph: *windspeedmph,
            yearlyrainin: *yearlyrainin,
        })
    }
}

const TIME_FORMAT: &'static [FormatItem<'static>] =
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");

fn try_into_time(dateutc: &str) -> Result<OffsetDateTime, time::error::Parse> {
    let parsed = PrimitiveDateTime::parse(dateutc, &TIME_FORMAT)?;

    Ok(OffsetDateTime::new_utc(parsed.date(), parsed.time()))
}

#[cfg(test)]
mod test {
    use time::{Date, Month, OffsetDateTime, Time};

    #[test]
    fn try_into_time() {
        let day = Date::from_calendar_date(2024, Month::January, 2).unwrap();
        let time = Time::from_hms(3, 4, 5).unwrap();
        let expected = OffsetDateTime::new_utc(day, time);

        let parsed = super::try_into_time("2024-01-02 03:04:05").expect("failed to parse time");

        assert_eq!(expected, parsed);
    }
}
