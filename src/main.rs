use anyhow::{Context, Result};
use clap::Parser;
use log::info;

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
    let reader = std::fs::File::open(file_path)
        .with_context(|| format!("Failed to open file for read: {}", file_path.display()))?;

    let mut buf = Vec::new();

    fix_lines::fix_content(reader, &mut buf)
        .with_context(|| format!("Failed to read content from file: {}", file_path.display()))?;

    std::fs::write(file_path, buf)
        .with_context(|| format!("Failed to write content to file: {}", file_path.display()))?;
    Ok(())
}
