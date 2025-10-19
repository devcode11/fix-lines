use std::io::Result;

#[cfg(windows)]
const TO_LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const TO_LINE_ENDING: &str = "\n";

pub fn fix_content(mut reader: impl std::io::Read, mut writer: impl std::io::Write) -> Result<()> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let fixed_content = fix_string(buf);
    writer.write_all(fixed_content.as_bytes())
}

fn fix_string(content: String) -> String {
    let mut fixed = content
        .trim_ascii_end()
        .lines()
        .map(|line| line.trim_ascii_end())
        .collect::<Vec<&str>>()
        .join(TO_LINE_ENDING)
        .to_string();
    fixed.push_str(TO_LINE_ENDING);
    fixed
}

#[test]
fn fix_some_string() {
    let input = String::from("# some heading\n\na saw ep\n\'spa\ns eqwe q                  qsdqq\n  dqqw e \r asd aaewe\na \nsda\n e            \r\njak\n\nasjalsejlasjea;laksd;a\na xd\nas d\n\n\n\nas dase alkj\n\n\r\n\nasase\n\n\n");
    let expected = String::from("# some heading\n\na saw ep\n\'spa\ns eqwe q                  qsdqq\n  dqqw e \r asd aaewe\na\nsda\n e\njak\n\nasjalsejlasjea;laksd;a\na xd\nas d\n\n\n\nas dase alkj\n\n\n\nasase\n");
    let actual = fix_string(input);
    assert_eq!(actual, expected);
}
