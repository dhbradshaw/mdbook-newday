use chrono::{Local, DateTime};

pub fn todays_line() -> String {
    let dt: DateTime<Local> = Local::now();
    let formatted = dt.format("- [%A, %b %d, %Y](./%Y/%Y-%m/%Y-%m-%d.md)");
    formatted.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", todays_line());
    }
}
