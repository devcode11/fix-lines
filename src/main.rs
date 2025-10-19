use anyhow::{Context, Result};

#[cfg(windows)]
const TO_LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const TO_LINE_ENDING: &str = "\n";

fn main() {
    let invoke_name = std::env::args().nth(0).expect("Missing zeroth argument");
    for arg in std::env::args().skip(1) {
        if arg == "--help" || arg == "-h" {
            print_help(invoke_name);
            return;
        }
        _ = process_file(arg.as_str());
    }
}

fn print_help(invoke_name: String) {
    println!(
"Fix line endings, remove empty lines at the end of the file and insert a final new line.\nusage: {} <file path>...",
    invoke_name
    );
}

fn process_file(file_path: &str) -> Result<()> {
    let read_content = std::fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path))?;

    let mut fixed = read_content
        .lines()
        .map(|line| line.trim_ascii_end())
        .collect::<Vec<&str>>()
        .join(TO_LINE_ENDING)
        .trim_ascii_end()
        .to_string();
    fixed.push_str(TO_LINE_ENDING);

    std::fs::write(file_path, fixed)
        .with_context(|| format!("Failed to update file: {}", file_path))?;
    Ok(())
}
