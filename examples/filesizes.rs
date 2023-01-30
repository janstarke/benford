use std::{path::PathBuf};

use anyhow::{bail, Result};
use benford::{BenfordTester, FirstDigitBase10};
use clap::{Parser};
use walkdir::WalkDir;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
/// scans all files in a directory and tests their filesize distribution
struct Cli {
    //// directories to scan
    directories: Vec<String>,
}
fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut tester = BenfordTester::default();
    let mut files = 0;

    for dir in cli.directories {
        let path = PathBuf::from(&dir);
        if !path.exists() {
            bail!("directory '{}' does not exist", dir);
        }

        if !path.is_dir() {
            bail!("path '{}' does not point to a directory", dir);
        }

        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.path_is_symlink() {
                continue;
            }
            let metadata = entry.metadata()?;
            if metadata.is_file() {
                if metadata.len() != 0 {
                    tester.add_sample::<FirstDigitBase10>(metadata.len().into());
                    files += 1;
                }
            }
        }
    }

    for d in 1..10 {
        let freq = (tester.as_ref()[d] as f64 * 100.0) / (files as f64);
        println!("frequency of {d}: {freq:.2}% ({} times)", tester.as_ref()[d])
    }

    println!("Chi² = {:.2}", tester.chi_squared());

    if tester.is_benford_with_alpha(benford::Alpha::Point75) {
        println!("the sizes match Benford's law :-o");
    } else {
        println!("the sizes do not match Benford's law :-(");
    }

    Ok(())
}
