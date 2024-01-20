#[derive(Debug)]
pub struct Gauge {
    name: String,
    labels: String,
    value: f64,
    timestamp: i64,
}

impl Gauge {
    pub fn new(
        name: impl Into<String>,
        labels: impl Into<String>,
        value: f64,
        timestamp: i64,
    ) -> Self {
        Self {
            name: name.into(),
            labels: labels.into(),
            value,
            timestamp,
        }
    }

    pub fn encode(&self) -> String {
        format!(
            "{}{{{}}} {} {}\n",
            self.name, self.labels, self.value, self.timestamp
        )
    }
}

impl Into<String> for &Gauge {
    fn into(self) -> String {
        let name = &self.name;
        let labels = &self.labels;
        let value = self.value;
        let timestamp = self.timestamp;

        format!("{name}{{{labels}}} {value} {timestamp}")
    }
}
