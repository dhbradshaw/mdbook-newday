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

/// Insert the new line right before the first line starting with sigil.
/// If there is no such line, insert line after all other lines.
/// If there is already a matching line, don't insert.
fn insert_line_before_sigil(line: &str, sigil: &str, text: &str) -> String {
    let mut new_lines = vec![];
    let mut sigil_found = false;
    let mut already_added = false;
    // Add the line before the first sigil if there is one.
    for text_line in text.lines() {
        if text_line == line {
            already_added = true;
        }
        if text_line.starts_with(sigil) && !sigil_found && !already_added {
            if text_line != line {
                new_lines.push(line);
                already_added = true;
            }
            sigil_found = true;
        }
        new_lines.push(text_line);
    }
    // Add the line at the end if there is no sigil present.
    if !sigil_found && !already_added {
        new_lines.push(line);
    }
    let mut with_insert = new_lines.join("\n");
    with_insert.push('\n');
    with_insert
}

fn add_line_to_file(line: &str, sigil: &str, file_path: &str) -> Result<(), std::io::Error> {
    let file_contents = std::fs::read_to_string(file_path)?;
    let file_contents = insert_line_before_sigil(line, sigil, &file_contents);
    std::fs::write(file_path, file_contents)
}

/// Adds a new line almost to the top of an mdbook summary page.
/// That is, not quite the top because it skips any lines without the `- [`
/// prefix like any title or commentary or introduction link.
/// 
/// The new line comes from local time and takes the format
/// `- [%A, %b %d, %Y](./%Y/%Y-%m/%Y-%m-%d.md)`, ie
/// `- [Thursday, Jan 01, 1970](./1970/1970-01/1970-01-01.md)`.
/// If there is no line yet, it will add one.
/// 
/// `mdbook serve` will create the files mentioned in the summary with titles
/// given by the link text.  So if this function is run right before `mdbook serve`
/// you'll get not only the summary link but also the file.
pub fn update_summary(path: &str) -> Result<(), std::io::Error> {
    add_line_to_file(&todays_line(), SIGIL, path)
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
        assert_eq!(contents, format!("{}\n{}\n", original_line, added_line));

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
        let text = insert_line_before_sigil(line, sigil, text_without_sigil);
        assert_eq!(text, "[Intro](./intro)\n- [First!](./first.md)\n");

        // Otherwise, the line is added before the first appearance of the sigil.
        let text = insert_line_before_sigil(line, sigil, text_with_sigil);
        assert_eq!(
            text,
            "[Intro](./intro)\n- [First!](./first.md)\n- [Second!](./second.md)\n"
        );

        // Idempotent
        let text = insert_line_before_sigil(line, sigil, &text);
        assert_eq!(
            text,
            "[Intro](./intro)\n- [First!](./first.md)\n- [Second!](./second.md)\n"
        );

        // Only add the line in one place even if the sigil appears multiple times.
        let text = insert_line_before_sigil(line, sigil, text_with_2_sigils);
        assert_eq!(text, "[Intro](./intro)\n- [First!](./first.md)\n- [\n- [\n");

        // A line without a sigil is still added idempotently.
        let first_pass = insert_line_before_sigil(line_without_sigil, sigil, text_without_sigil);
        assert_eq!(first_pass, "[Intro](./intro)\nno sigil\n");
        let second_pass = insert_line_before_sigil(line_without_sigil, sigil, &first_pass);
        assert_eq!(first_pass, second_pass)
    }

    #[test]
    fn test_gives_summary_terminal_newline() {
        let sigil = "- [";
        let without_sigil = "[Introduction](introduction.md)";
        let with_sigil = "- [a](a.md)";
        let line = "- [Thursday, Jan 01, 1970](./1970/1970-01/1970-01-01.md)";
        
        let expected = format!("{without_sigil}\n{line}\n{with_sigil}\n");
        
        // If the terminal newline is already there, we don't lose it.
        let text = format!("{without_sigil}\n{with_sigil}\n");
        let with_insert = insert_line_before_sigil(line, sigil, &text);
        assert_eq!(with_insert, expected);

        // If it was missing, we gain it.
        let text = format!("{without_sigil}\n{with_sigil}");
        let with_insert = insert_line_before_sigil(line, sigil, &text);
        assert_eq!(with_insert, expected);

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
        // There's no fixed value here because it depends on the time and local timezone.
        // So rather than create a test this just allows for quick inspection.
        dbg!(todays_line());
    }
    #[test]
    fn test_lines() {
        let lines = "a\n";
        for line in lines.lines() {
            println!("Here's the line: {line:?}")
        }
    }
}
