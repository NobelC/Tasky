use crate::backend::task::TimeStatusTask;
use chrono::{DateTime, NaiveDate, Utc};

pub(super) fn convert_date(date_raw: &str) -> DateTime<Utc> {
    NaiveDate::parse_from_str(date_raw, "%d-%m-%Y")
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .expect("Error")
        .and_utc()
}
pub(super) fn calculate_status(end_time: DateTime<Utc>) -> TimeStatusTask {
    let rest_time = end_time - Utc::now();
    if rest_time.num_days() < 0 {
        TimeStatusTask::Overdue
    } else if rest_time.num_days() < 4 {
        TimeStatusTask::AboutToExpire
    } else {
        TimeStatusTask::OnTime
    }
}
