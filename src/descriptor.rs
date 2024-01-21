use crate::Encoder;

enum MetricType {
    Gauge,
    Info,
}

pub struct Descriptor {
    name: String,
    metric_type: MetricType,
    description: String,
    unit: Option<String>,
}

impl Descriptor {
    pub fn gauge(
        name: impl Into<String>,
        description: impl Into<String>,
        unit: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            metric_type: MetricType::Gauge,
            description: description.into(),
            unit: Some(unit.into()),
        }
    }

    pub fn info(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            metric_type: MetricType::Info,
            description: description.into(),
            unit: None,
        }
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), std::fmt::Error> {
        encoder.encode_descriptor(self)
    }

    pub fn name(&self) -> String {
        match self.metric_type {
            MetricType::Gauge => self.name.clone(),
            MetricType::Info => format!("{}_info", self.name),
        }
    }

    pub fn unit(&self) -> Option<&str> {
        self.unit.as_deref()
    }
}
