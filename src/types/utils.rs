use chrono::NaiveDateTime;

pub fn parse_date_from_row(row: Option<String>) -> Option<NaiveDateTime> {
    row.map(|date| NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M:%S").ok())
        .unwrap_or(None)
}
