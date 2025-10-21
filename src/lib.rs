use std::io::Result;

#[cfg(windows)]
pub const DEFAULT_LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
pub const DEFAULT_LINE_ENDING: &str = "\n";

pub fn fix_content(mut reader: impl std::io::Read, mut writer: impl std::io::Write) -> Result<()> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let fixed_content = fix_string(buf);
    writer.write_all(fixed_content.as_bytes())
}

fn fix_string(content: String) -> String {
    content
        .trim_ascii_end()
        .lines()
        .map(|line| line.trim_ascii_end())
        .collect::<Vec<&str>>()
        .join(DEFAULT_LINE_ENDING)
        .to_string()
        + DEFAULT_LINE_ENDING
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_trailing_spaces() {
        let input = "a  ".to_string();
        let expected = format!("a{DEFAULT_LINE_ENDING}");
        let actual = fix_string(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn add_final_newline() {
        let input = "a".to_string();
        let expected = format!("a{DEFAULT_LINE_ENDING}");
        let actual = fix_string(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn change_line_ending() {
        let input = "a\n".to_string();
        let expected = format!("a{DEFAULT_LINE_ENDING}");
        let actual = fix_string(input);
        assert_eq!(actual, expected);
    }
}
