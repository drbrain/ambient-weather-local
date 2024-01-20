#[derive(Debug)]
pub struct Info {
    name: String,
    labels: String,
    timestamp: i64,
}

impl Info {
    pub fn new(name: impl Into<String>, labels: impl Into<String>, timestamp: i64) -> Self {
        Self {
            name: name.into(),
            labels: labels.into(),
            timestamp,
        }
    }
}

impl Into<String> for &Info {
    fn into(self) -> String {
        let name = &self.name;
        let labels = &self.labels;
        let timestamp = self.timestamp;

        format!("{name}{{{labels}}} 1 {timestamp}")
    }
}
