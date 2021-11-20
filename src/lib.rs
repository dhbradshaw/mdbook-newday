use chrono::{DateTime, Local, TimeZone};

const SIGIL: &str = "- [";

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

fn todays_line() -> String {
    mdbook_summary_line_for_time(now())
}

fn place_line_before(new_line: &str, sigil: &str, text: &str) -> String {
    let mut new_lines = vec![];
    let mut sigil_found = false;
    let mut already_added = false;
    // Add the line before the first sigil if there is one.
    for text_line in text.lines() {
        if text_line == new_line {
            already_added = true;
        }
        if text_line.starts_with(sigil) && !sigil_found && !already_added {
            if text_line != new_line {
                new_lines.push(new_line);
                already_added = true;
            }
            sigil_found = true;
        }
        new_lines.push(text_line);
    }
    // Add the line at the end if there is no sigil present.
    if !sigil_found && !already_added {
        new_lines.push(new_line);
    }
    new_lines.join("\n")
}

fn add_line_to_file(file_path: &str, sigil: &str, line: &str) -> Result<(), std::io::Error> {
    let file_contents = std::fs::read_to_string(file_path)?;
    let file_contents = place_line_before(line, sigil, &file_contents);
    std::fs::write(file_path, file_contents)?;
    Ok(())
}

pub fn update_summary(path: &str) -> Result<(), std::io::Error> {
    add_line_to_file(path, SIGIL, &todays_line())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_update_summary() -> Result<(), std::io::Error> {
        let tmp_path = "./tmp_file";
        let original_line = "original_line";
        
        // Create the file with only the original line.
        std::fs::write(tmp_path, original_line)?;
        
        // Add in the new line.
        update_summary(tmp_path)?;
        
        // Check file contents.
        let added_line = &todays_line();
        let contents = std::fs::read_to_string(tmp_path)?;
        assert_eq!(contents, format!("{}\n{}", original_line, added_line));

        // Now delete the file again.
        std::fs::remove_file(tmp_path)?;
        Ok(())
    }

    #[test]
    fn test_place_line_before() {
        let line = "- [First!](./first.md)";
        let line_without_sigil = "no sigil";
        let sigil = "- [";
        let text_without_sigil = "[Intro](./intro)";
        let text_with_sigil = "[Intro](./intro)\n- [Second!](./second.md)";
        let text_with_2_sigils = "[Intro](./intro)\n- [\n- [";

        // If there is no sigil, the line is added to the end of the file.
        let text = place_line_before(
            line,
            sigil,
            text_without_sigil,
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
        
        // A line without a sigil is still added idempotently.
        let first_pass = place_line_before(
            line_without_sigil,
            sigil,
            text_without_sigil,
        );
        assert_eq!(first_pass, "[Intro](./intro)\nno sigil");
        let second_pass = place_line_before(
            line_without_sigil,
            sigil,
            &first_pass,
        );
        assert_eq!(first_pass, second_pass)

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
