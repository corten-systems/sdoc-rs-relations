use anyhow::{bail, Context, Result};
use clap::Parser;

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use std::io::{self, Write};

mod sdoc;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the crate root directory
    #[arg(short = 'c', long = "crate", value_name = "PATH", default_value = ".")]
    path: PathBuf,
    /// Output file (use '-' or omit to write to stdout)
    #[arg(short = 'o', long = "output", value_name = "FILE", default_value = "-")]
    output: PathBuf,
}

fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Validate that the provided path exists and is a directory
    if !args.path.exists() {
        bail!("path does not exist: {}", args.path.display());
    }
    if !args.path.is_dir() {
        bail!("path is not a directory: {}", args.path.display());
    }

    // Find Rust source files and process them
    let files = find_rust_files(&args.path)?;
    let mut relationships = BTreeMap::new();
    for file in files {
        let relations = sdoc::find_relations(&file)?;
        relationships.insert(file, relations);
    }

    // Serialize relationships to JSON and write to the requested output
    if args.output.as_os_str() == OsStr::new("-") {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        serde_json::to_writer_pretty(&mut handle, &relationships)?;
        handle.write_all(b"\n").ok();
    } else {
        let file = fs::File::create(&args.output)
            .with_context(|| format!("failed to create output file: {}", args.output.display()))?;
        let mut writer = io::BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &relationships)?;
        writer.write_all(b"\n").ok();
    }

    Ok(())
}

/// Collect all Rust source files ("*.rs") starting from `root` and all
/// subdirectories. This function does not follow symbolic links.
fn find_rust_files<P: AsRef<Path>>(root: P) -> Result<Vec<PathBuf>> {
    let root = root.as_ref();
    if !root.is_dir() {
        bail!("root path is not a directory: {}", root.display());
    }

    let mut files: Vec<PathBuf> = Vec::new();
    let mut stack: Vec<PathBuf> = vec![root.to_path_buf()];

    while let Some(dir) = stack.pop() {
        let read_dir = fs::read_dir(&dir)
            .with_context(|| format!("failed to read directory: {}", dir.display()))?;
        for entry in read_dir {
            let entry = entry.with_context(|| {
                format!("failed to access entry in directory: {}", dir.display())
            })?;
            let file_type = entry.file_type().with_context(|| {
                format!("failed to get file type for: {}", entry.path().display())
            })?;

            if file_type.is_symlink() {
                // Do not follow symbolic links
                continue;
            } else if file_type.is_dir() {
                stack.push(entry.path());
            } else if file_type.is_file()
                && entry.path().extension().and_then(|s| s.to_str()) == Some("rs")
            {
                files.push(entry.path());
            }
        }
    }

    // Sort for deterministic order
    files.sort();
    Ok(files)
}
