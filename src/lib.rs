use chrono::{DateTime, Local, TimeZone};

fn now() -> DateTime<Local> {
    Local::now()
}

fn mdbook_summary_line_for_time<T: TimeZone>(dt: DateTime<T>) -> String
where
    T::Offset: std::fmt::Display,
{
    dt.format("- [%A, %b %d, %Y](./%Y/%Y-%m/%Y-%m-%d.md)")
        .to_string()
}

pub fn todays_line() -> String {
    mdbook_summary_line_for_time(now())
}

fn place_line_before(new_line: &str, sigil: &str, text: &str) -> String {
    let mut new_lines = vec![];
    for text_line in text.lines() {
        if text_line.starts_with(sigil) {
            new_lines.push(new_line);
        }
        new_lines.push(text_line);
    }
    new_lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_place_line_before() {
        let text = place_line_before(
            "- [First!](./first.md)",
            "- [",
            "[Intro](./intro)\n- [Second!](./second.md)",
        );
        assert_eq!(text, "[Intro](./intro)\n- [First!](./first.md)\n- [Second!](./second.md)");
    }

    #[test]
    fn test_summary_line_format() {
        let dt: DateTime<Utc> = Utc.timestamp(1, 0);
        let formatted = mdbook_summary_line_for_time(dt);
        assert_eq!(
            formatted,
            "- [Thursday, Jan 01, 1970](./1970/1970-01/1970-01-01.md)"
        );
    }

    #[test]
    fn test_todays_line() {
        dbg!(todays_line());
    }
}
