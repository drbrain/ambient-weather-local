use crate::{Encoder, Metric};
use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq)]
pub struct Gauge {
    metric: Metric,
    value: f64,
    timestamp: Option<i64>,
}

impl Gauge {
    pub fn new(name: impl Into<String>, labels: impl Into<String>, value: f64) -> Self {
        let metric = Metric::new(name, labels);

        Self {
            metric,
            value,
            timestamp: None,
        }
    }

    pub fn new_with_timestamp(
        name: impl Into<String>,
        labels: impl Into<String>,
        value: f64,
        timestamp: i64,
    ) -> Self {
        let metric = Metric::new(name, labels);

        Self {
            metric,
            value,
            timestamp: Some(timestamp),
        }
    }

    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), std::fmt::Error> {
        encoder.encode_gauge(self)
    }

    pub fn metric(&self) -> &Metric {
        &self.metric
    }

    pub fn timestamp(&self) -> Option<i64> {
        self.timestamp
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

impl Eq for Gauge {}

impl Ord for Gauge {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.metric.cmp(&other.metric) {
            Ordering::Equal => {}
            ord => return ord,
        }

        match (self.timestamp, other.timestamp) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Greater,
            (Some(_), None) => Ordering::Less,
            (Some(timestamp), Some(other_timestamp)) => timestamp.cmp(&other_timestamp),
        }
    }
}

impl PartialOrd for Gauge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.metric.partial_cmp(&other.metric) {
            Some(Ordering::Equal) => {}
            ord => return ord,
        }

        match (self.timestamp, other.timestamp) {
            (None, None) => Some(Ordering::Equal),
            (None, Some(_)) => Some(Ordering::Greater),
            (Some(_), None) => Some(Ordering::Less),
            (Some(timestamp), Some(other_timestamp)) => timestamp.partial_cmp(&other_timestamp),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cmp() {
        let a = Gauge::new("a", "", 0.0);
        let a_1 = Gauge::new_with_timestamp("a", "", 0.0, 1);
        let a_2 = Gauge::new_with_timestamp("a", "", 1.0, 2);

        let b = Gauge::new("b", "", 0.0);
        let b_1 = Gauge::new_with_timestamp("b", "", 0.0, 1);

        assert_eq!(Ordering::Equal, a.cmp(&a));
        assert_eq!(Ordering::Equal, a_1.cmp(&a_1));

        assert_eq!(Ordering::Less, a.cmp(&b));
        assert_eq!(Ordering::Greater, b.cmp(&a));

        assert_eq!(Ordering::Greater, a.cmp(&a_1));
        assert_eq!(Ordering::Less, a_1.cmp(&a_2));
        assert_eq!(Ordering::Less, a_2.cmp(&a));

        assert_eq!(Ordering::Less, a.cmp(&b_1));
        assert_eq!(Ordering::Less, a_1.cmp(&b_1));
    }

    #[test]
    fn partial_cmp() {
        let a = Gauge::new("a", "", 0.0);
        let a_1 = Gauge::new_with_timestamp("a", "", 0.0, 1);
        let a_2 = Gauge::new_with_timestamp("a", "", 1.0, 2);

        let b = Gauge::new("b", "", 0.0);
        let b_1 = Gauge::new_with_timestamp("b", "", 0.0, 1);

        assert_eq!(Some(Ordering::Equal), a.partial_cmp(&a));
        assert_eq!(Some(Ordering::Equal), a_1.partial_cmp(&a_1));

        assert_eq!(Some(Ordering::Less), a.partial_cmp(&b));
        assert_eq!(Some(Ordering::Greater), b.partial_cmp(&a));

        assert_eq!(Some(Ordering::Greater), a.partial_cmp(&a_1));
        assert_eq!(Some(Ordering::Less), a_1.partial_cmp(&a_2));
        assert_eq!(Some(Ordering::Less), a_2.partial_cmp(&a));

        assert_eq!(Some(Ordering::Less), a.partial_cmp(&b_1));
        assert_eq!(Some(Ordering::Less), a_1.partial_cmp(&b_1));
    }
}
