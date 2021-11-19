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
    use chrono::TimeZone;

    #[test]
    fn summary_line() {
        let dt: DateTime<Local> = Local.timestamp(1, 0);
        println!("{}", mdbook_summary_line_for_time(dt));
        println!("Today's line: {}", todays_line());

    }
}
