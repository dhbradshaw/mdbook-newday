use chrono::{Local, DateTime, TimeZone};

pub fn now() -> DateTime<Local>{
    Local::now()
}

pub fn mdbook_summary_line_for_time<T: TimeZone>(dt: DateTime<T>) -> String 
    where T::Offset: std::fmt::Display {
    dt.format("- [%A, %b %d, %Y](./%Y/%Y-%m/%Y-%m-%d.md)").to_string()
}

pub fn todays_line() -> String {
    mdbook_summary_line_for_time(now())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_summary_line_format() {
        let dt: DateTime<Utc> = Utc.timestamp(1, 0);
        let formatted = mdbook_summary_line_for_time(dt);
        assert_eq!(formatted, "- [Thursday, Jan 01, 1970](./1970/1970-01/1970-01-01.md)");
    }

    #[test]
    fn test_todays_line() {
        dbg!(todays_line());
    }
}
