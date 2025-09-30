use anyhow::Result;

use clap::Parser;
use either::Either;
use walkdir::WalkDir;

use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

mod parse;
mod sdoc;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the crate root directory
    #[arg(short = 'c', long = "crate", value_name = "PATH", default_value = ".")]
    path: PathBuf,
    /// Output file (use '-' or omit for output to stdout)
    #[arg(short = 'o', long = "output", value_name = "FILE", default_value = "-")]
    output: PathBuf,
}

pub fn reader_for(path: &OsStr) -> io::Result<impl BufRead> {
    Ok(if path == "-" {
        Either::Left(io::stdin().lock())
    } else {
        Either::Right(io::BufReader::new(File::open(path)?))
    })
}

pub fn writer_for(path: &OsStr) -> io::Result<impl Write> {
    Ok(if path == "-" {
        Either::Left(io::stdout().lock())
    } else {
        Either::Right(io::BufWriter::new(File::open(path)?))
    })
}

fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Find Rust source files and process them
    let files = find_rust_files(&args.path)?;
    let mut relationships = BTreeMap::new();
    for file in files {
        let relations = sdoc::find_relations(&file, &args.path)?;
        relationships.insert(file, relations);
    }

    let mut writer = writer_for(args.output.as_os_str())?;
    serde_json::to_writer_pretty(&mut writer, &relationships)?;
    writer.write_all(b"\n")?;

    Ok(())
}

/// Collect all Rust source files ("*.rs") starting from `root` and all
/// subdirectories. This function does not follow symbolic links with the
/// exception if the root is a symbolic link itself.
fn find_rust_files<P: AsRef<Path>>(root: P) -> Result<Vec<PathBuf>> {
    let walker = WalkDir::new(root)
        .follow_root_links(true)
        .follow_links(false)
        .into_iter();
    let mut result = vec![];
    let rs_extension = Some(OsStr::new("rs"));
    for entry in walker {
        let path = entry?.into_path();
        if path.is_file() && path.extension() == rs_extension {
            result.push(path);
        }
    }
    result.sort_unstable();
    result.dedup();
    Ok(result)
}
