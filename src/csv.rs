use std::fs;
use std::io::Write;
use std::path::Path;

/// Writes a CSV file with one header row and any number of text columns per row.
pub fn write_csv(path: &str, headers: &[&str], rows: &[Vec<String>]) -> std::io::Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    let mut file = fs::File::create(path)?;
    writeln!(file, "{}", headers.join(","))?;
    for row in rows {
        writeln!(file, "{}", row.join(","))?;
    }
    Ok(())
}
