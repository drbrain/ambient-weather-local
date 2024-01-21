/// The name and labels of a metric
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Metric {
    name: String,
    labels: String,
}

impl Metric {
    pub fn new(name: impl Into<String>, labels: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            labels: labels.into(),
        }
    }

    pub fn labels(&self) -> &str {
        &self.labels
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
