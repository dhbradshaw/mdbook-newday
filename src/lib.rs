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

pub fn place_line_before(new_line: &str, sigil: &str, text: &str) -> String {
    let mut new_lines = vec![];
    let mut sigil_found = false;
    // Add the line before the first sigil if there is one.
    for text_line in text.lines() {
        if text_line.starts_with(sigil) && !sigil_found {
            if text_line != new_line {
                new_lines.push(new_line);
            }
            sigil_found = true;
        }
        new_lines.push(text_line);
    }
    // Add the line at the end if there is no sigil present.
    if !sigil_found {
        new_lines.push(new_line);
    }
    new_lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_place_line_before() {
        let line = "- [First!](./first.md)";
        let sigil = "- [";
        let text_no_sigil = "[Intro](./intro)";
        let text_with_sigil = "[Intro](./intro)\n- [Second!](./second.md)";
        let text_with_2_sigils = "[Intro](./intro)\n- [\n- [";

        // If there is no sigil, the line is added to the end of the file.
        let text = place_line_before(
            line,
            sigil,
            text_no_sigil,
        );
        assert_eq!(text, "[Intro](./intro)\n- [First!](./first.md)");

        // Otherwise, the line is added before the first appearance of the sigil.
        let text = place_line_before(
            line,
            sigil,
            text_with_sigil,
        );
        assert_eq!(text, "[Intro](./intro)\n- [First!](./first.md)\n- [Second!](./second.md)");

        // Idempotent
        let text = place_line_before(
            line,
            sigil,
            &text,
        );
        assert_eq!(text, "[Intro](./intro)\n- [First!](./first.md)\n- [Second!](./second.md)");

        // Only add the line in one place even if the sigil appears multiple times.
        let text = place_line_before(
            line,
            sigil,
            text_with_2_sigils,
        );
        assert_eq!(text, "[Intro](./intro)\n- [First!](./first.md)\n- [\n- [");
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
