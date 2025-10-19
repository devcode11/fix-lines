use anyhow::{Context, Result};
use clap::Parser;
use log::info;

#[cfg(windows)]
const TO_LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const TO_LINE_ENDING: &str = "\n";

/// Fix line endings, remove empty lines at the end of the file and insert a final new line.
#[derive(Debug, Parser)]
#[command(version, about, long_about=None)]
struct CliArgs {
    /// Files to fix
    #[arg(required = true)]
    file_paths: Vec<std::path::PathBuf>,

    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

fn main() -> Result<()> {
    let args = CliArgs::parse();
    env_logger::Builder::new()
        .filter_level(args.verbosity.into())
        .init();

    info!("{:?}", args);

    for path in args.file_paths {
        _ = process_file(path.as_path())?;
    }
    Ok(())
}

fn process_file(file_path: &std::path::Path) -> Result<()> {
    info!("Processing file {:?}", file_path);
    let content = std::fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

    std::fs::write(file_path, fix_content(content))
        .with_context(|| format!("Failed to update file: {}", file_path.display()))?;
    Ok(())
}

fn fix_content(content: String) -> String {
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
    let actual = fix_content(input);
    assert_eq!(actual, expected);
}
