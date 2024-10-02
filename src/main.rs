use chrono::Datelike;
use clap::Parser;

use std::{fs, io::Result, path::PathBuf};

/// MIT License Generator
#[derive(Parser, Debug)]
struct Cli {
    /// Name to use for the MIT License
    #[arg(short, long)]
    name: String,

    /// Output directory for the MIT License.
    /// Defaults to the current directory
    #[arg(short, long)]
    output: Option<String>,
}

fn get_license(name: &str) -> String {
    let year = chrono::Utc::now().year();
    format!(
        r#"MIT License

Copyright (c) {year} {name}

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE."#
    )
}

fn write_license(license: &str, destination: Option<String>) -> Result<()> {
    let dst = match destination {
        Some(d) => PathBuf::from(d).join("LICENSE-MIT"),
        None => PathBuf::from(".").join("LICENSE-MIT"),
    };

    if !dst.exists() {
        fs::write(dst, license)?;
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    let license = get_license(&cli.name);
    write_license(&license, cli.output).expect("unable to write license to disk");
}
