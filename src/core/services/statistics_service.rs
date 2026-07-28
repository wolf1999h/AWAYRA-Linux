use chrono::Utc;
use crate::core::models::{BreakType, DailyStatistics, StatisticsData};

pub struct StatisticsService {
    data: StatisticsData,
}

impl StatisticsService {
    pub fn new(data: StatisticsData) -> Self {
        Self { data }
    }

    pub fn data(&self) -> &StatisticsData {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut StatisticsData {
        &mut self.data
    }

    pub fn get_today(&self) -> DailyStatistics {
        let today = Utc::now().date_naive();
        self.data.get_today(today)
    }

    pub fn record_completion(&mut self, break_type: BreakType) {
        let today = Utc::now().date_naive();
        self.data.record_completion(today, break_type);
    }

    pub fn record_skip(&mut self) {
        let today = Utc::now().date_naive();
        self.data.record_skip(today);
    }

    pub fn record_snooze(&mut self) {
        let today = Utc::now().date_naive();
        self.data.record_snooze(today);
    }
}