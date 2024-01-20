use crate::{report::Report, Metrics};
use std::{collections::VecDeque, sync::RwLock};

/// Minimum interval for Ambient weather devices is 16 seconds
///
/// This holds about 5 minutes of reports with the minimum interval
const MAX_REPORTS: usize = 20;

#[derive(Default)]
pub struct Reports {
    reports: RwLock<VecDeque<Metrics>>,
}

impl Reports {
    pub fn add_report(&self, report: Report) {
        let metrics = report.into();

        let mut reports = self.reports.write().unwrap();

        reports.push_back(metrics);

        if reports.len() > MAX_REPORTS {
            reports.pop_front();
        }
    }

    pub fn metrics(&self) -> String {
        let reports = self.reports.read().unwrap();

        reports
            .iter()
            .map(|report| report.into())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
