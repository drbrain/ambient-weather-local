use crate::ambient_weather_report::AmbientWeatherReport;
use time::{
    format_description::FormatItem, macros::format_description, OffsetDateTime, PrimitiveDateTime,
};

#[derive(Debug)]
pub struct Report {
    pub station_type: String,
    pub passkey: String,
    pub time: OffsetDateTime,
    pub aqi_pm25: Option<f64>,
    pub aqi_pm25_24h: Option<f64>,
    pub aqi_pm25_24h_aqin: Option<f64>,
    pub aqi_pm25_aqin: Option<f64>,
    pub baromabsin: Option<f64>,
    pub baromrelin: Option<f64>,
    pub batt1: Option<f64>,
    pub batt2: Option<f64>,
    pub batt3: Option<f64>,
    pub batt4: Option<f64>,
    pub batt5: Option<f64>,
    pub batt6: Option<f64>,
    pub batt7: Option<f64>,
    pub batt8: Option<f64>,
    pub batt_25: Option<f64>,
    pub batt_co2: Option<f64>,
    pub batt_lightning: Option<f64>,
    pub battin: Option<f64>,
    pub battout: Option<f64>,
    pub battsm1: Option<f64>,
    pub battsm2: Option<f64>,
    pub battsm3: Option<f64>,
    pub battsm4: Option<f64>,
    pub battsm5: Option<f64>,
    pub battsm6: Option<f64>,
    pub battsm7: Option<f64>,
    pub battsm8: Option<f64>,
    pub co2_in_24h_aqin: Option<f64>,
    pub co2_in_aqin: Option<f64>,
    pub dailyrainin: Option<f64>,
    pub eventrainin: Option<f64>,
    pub hourlyrainin: Option<f64>,
    pub humidity: Option<f64>,
    pub humidityin: Option<f64>,
    pub lightning_day: Option<f64>,
    pub maxdailygust: Option<f64>,
    pub monthlyrainin: Option<f64>,
    pub pm10_in_24h_aqin: Option<f64>,
    pub pm10_in_aqin: Option<f64>,
    pub pm25: Option<f64>,
    pub pm25_24h: Option<f64>,
    pub pm25_in_24h_aqin: Option<f64>,
    pub pm25_in_aqin: Option<f64>,
    pub pm_in_humidity_aqin: Option<f64>,
    pub pm_in_temp_aqin: Option<f64>,
    pub soilhum1: Option<f64>,
    pub soilhum2: Option<f64>,
    pub soilhum3: Option<f64>,
    pub soilhum4: Option<f64>,
    pub soilhum5: Option<f64>,
    pub soilhum6: Option<f64>,
    pub soilhum7: Option<f64>,
    pub soilhum8: Option<f64>,
    pub solarradiation: Option<f64>,
    pub tempf: Option<f64>,
    pub tempinf: Option<f64>,
    pub uv: Option<f64>,
    pub weeklyrainin: Option<f64>,
    pub winddir: Option<f64>,
    pub winddir_avg10m: Option<f64>,
    pub windgustmph: Option<f64>,
    pub windspdmph_avg10m: Option<f64>,
    pub windspeedmph: Option<f64>,
    pub yearlyrainin: Option<f64>,
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

impl PartialEq for Report {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl PartialOrd for Report {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.time.partial_cmp(&other.time)
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
