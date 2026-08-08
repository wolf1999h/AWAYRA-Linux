use chrono::{NaiveDate};
use serde::{Deserialize, Serialize};
use crate::core::models::BreakType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyStatistics {
    pub date: NaiveDate,
    pub eye_completed: i32,
    pub move_completed: i32,
    pub skipped: i32,
    pub snoozed: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticsData {
    pub days: Vec<DailyStatistics>,
}

impl StatisticsData {
    pub fn create_default() -> Self {
        Self { days: Vec::new() }
    }

    pub fn get_today(&self, today: NaiveDate) -> DailyStatistics {
        self.days
            .iter()
            .find(|d| d.date == today)
            .cloned()
            .unwrap_or(DailyStatistics {
                date: today,
                eye_completed: 0,
                move_completed: 0,
                skipped: 0,
                snoozed: 0,
            })
    }

    pub fn record_completion(&mut self, today: NaiveDate, break_type: BreakType) {
        let idx = self.find_or_create_index(today);
        match break_type {
            BreakType::Eye => self.days[idx].eye_completed += 1,
            BreakType::Move => self.days[idx].move_completed += 1,
        }
    }

    pub fn record_skip(&mut self, today: NaiveDate) {
        let idx = self.find_or_create_index(today);
        self.days[idx].skipped += 1;
    }

    pub fn record_snooze(&mut self, today: NaiveDate) {
        let idx = self.find_or_create_index(today);
        self.days[idx].snoozed += 1;
    }

    fn find_or_create_index(&mut self, today: NaiveDate) -> usize {
        if let Some(cutoff) = today.checked_sub_signed(chrono::Duration::days(365)) {
            self.days.retain(|d| d.date >= cutoff);
        }

        if let Some(idx) = self.days.iter().position(|d| d.date == today) {
            return idx;
        }
        self.days.push(DailyStatistics {
            date: today,
            eye_completed: 0,
            move_completed: 0,
            skipped: 0,
            snoozed: 0,
        });
        self.days.len() - 1
    }
}