use crate::report::Report;
use std::{collections::VecDeque, sync::RwLock};

/// Minimum interval for Ambient weather devices is 16 seconds
///
/// This holds 16 minutes of
const MAX_REPORTS: usize = 60;

#[derive(Default)]
pub struct Reports {
    reports: RwLock<VecDeque<Report>>,
}

impl Reports {
    pub fn add_report(&self, report: impl Into<Report>) {
        let mut reports = self.reports.write().unwrap();

        reports.push_back(report.into());

        if reports.len() > MAX_REPORTS {
            reports.pop_front();
        }
    }
}
