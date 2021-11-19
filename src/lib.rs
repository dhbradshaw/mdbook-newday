use chrono::{Local, DateTime};

pub fn now() -> DateTime<Local>{
    Local::now()
}

pub fn mdbook_summary_line_for_time(t: DateTime<Local>) -> String {
    t.format("- [%A, %b %d, %Y](./%Y/%Y-%m/%Y-%m-%d.md)").to_string()
}

pub fn todays_line() -> String {
    mdbook_summary_line_for_time(now())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", todays_line());
    }
}
