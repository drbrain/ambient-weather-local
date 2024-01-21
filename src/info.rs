use crate::{Encoder, Metric};

#[derive(Clone, Debug)]
pub struct Info {
    metric: Metric,
    timestamp: i64,
}

impl Info {
    pub fn new(name: impl Into<String>, labels: impl Into<String>, timestamp: i64) -> Self {
        let metric = Metric::new(name, labels);

        Self { metric, timestamp }
    }

    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), std::fmt::Error> {
        encoder.encode_info(self)
    }

    pub fn metric(&self) -> &Metric {
        &self.metric
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }
}
