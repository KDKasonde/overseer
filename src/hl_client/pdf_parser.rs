use crate::overseer::structs::HistoricalTransaction;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;
use std::process::Command;

/// Response structure matching Python parser's JSON output
#[derive(Debug, Deserialize)]
struct TransactionsResponse {
    transactions: Vec<HistoricalTransaction>,
}

/// Parses Hargreaves Lansdown PDF using Python pdfplumber-based parser.
///
/// This function calls the external Python script that uses pdfplumber
/// for robust table extraction. See python_pdf_parser/RUST_INTEGRATION.md
/// for details on the integration.
///
/// # Arguments
///
/// * `path` - Path to the PDF file to parse
///
/// # Returns
///
/// A vector of `HistoricalTransaction` objects extracted from the
/// "CAPITAL ACCOUNT TRANSACTIONS" table in the PDF.
///
/// # Security
///
/// - For file paths, passes the path directly to the Python script
/// - For HTTP/network data, use `parse_hl_pdf_from_bytes` with secure temp files
///
/// # Example
///
/// ```no_run
/// use overseer::hl_client::pdf_parser::parse_hl_pdf;
///
/// let transactions = parse_hl_pdf("Investment Report.pdf")?;
/// println!("Parsed {} transactions", transactions.len());
/// ```
pub fn parse_hl_pdf<P: AsRef<Path>>(path: P) -> Result<Vec<HistoricalTransaction>> {
    let pdf_path = path.as_ref();

    // Get the project root directory (assuming we're in src/hl_client/)
    let current_dir = std::env::current_dir()
        .context("Failed to get current directory")?;

    let python_script = current_dir.join("python_pdf_parser/src/main.py");
    let python_venv = current_dir.join("python_pdf_parser/.venv/bin/python3");

    // Verify Python script exists
    if !python_script.exists() {
        return Err(anyhow::anyhow!(
            "Python parser script not found at {:?}. Expected: {:?}",
            python_script,
            current_dir.join("python_pdf_parser/src/main.py")
        ));
    }

    // Determine which Python interpreter to use
    // Priority: 1. Virtual environment python, 2. System python3
    let python_cmd = if python_venv.exists() {
        python_venv
    } else {
        std::path::PathBuf::from("python3")
    };

    // Call Python parser
    // Command: python python_pdf_parser/src/main.py <pdf_path> --json --table "CAPITAL ACCOUNT TRANSACTIONS"
    let output = Command::new(&python_cmd)
        .arg(&python_script)
        .arg(pdf_path)
        .arg("--json")
        .arg("--table")
        .arg("CAPITAL ACCOUNT TRANSACTIONS")
        .output()
        .context("Failed to execute Python PDF parser")?;

    // Check if command succeeded
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!(
            "Python parser failed with status {:?}: {}",
            output.status.code(),
            stderr
        ));
    }

    // Parse JSON output
    let json_str = String::from_utf8(output.stdout)
        .context("Python parser output is not valid UTF-8")?;

    let response: TransactionsResponse = serde_json::from_str(&json_str)
        .context("Failed to deserialize JSON from Python parser")?;

    Ok(response.transactions)
}

#[cfg(feature = "secure-temp-files")]
use std::io::Write;
#[cfg(feature = "secure-temp-files")]
use tempfile::NamedTempFile;

/// Parses Hargreaves Lansdown PDF from bytes using secure temporary file.
///
/// This function is recommended when processing PDFs downloaded from HTTP/network
/// sources or when handling sensitive financial data. It creates a secure temporary
/// file with restrictive permissions (0o600 on Unix) and automatically cleans up.
///
/// # Arguments
///
/// * `pdf_bytes` - The PDF file contents as bytes
///
/// # Returns
///
/// A vector of `HistoricalTransaction` objects extracted from the PDF.
///
/// # Security Features
///
/// - Creates temporary file with random, unpredictable name
/// - Sets owner-only permissions (0o600 on Unix)
/// - Automatic cleanup via RAII when function returns
/// - Prevents race conditions through atomic file creation
///
/// # Example
///
/// ```no_run
/// use overseer::hl_client::pdf_parser::parse_hl_pdf_from_bytes;
///
/// let pdf_bytes = std::fs::read("report.pdf")?;
/// let transactions = parse_hl_pdf_from_bytes(&pdf_bytes)?;
/// ```
///
/// # Note
///
/// Requires the "secure-temp-files" feature and the `tempfile` crate.
/// See python_pdf_parser/SECURE_TEMP_FILES.md for security details.
#[cfg(feature = "secure-temp-files")]
pub fn parse_hl_pdf_from_bytes(pdf_bytes: &[u8]) -> Result<Vec<HistoricalTransaction>> {
    // Create secure temporary file
    let mut temp_file = NamedTempFile::new()
        .context("Failed to create secure temporary file")?;

    // Set restrictive permissions: owner read/write only (0o600)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = temp_file.as_file().metadata()
            .context("Failed to get temp file metadata")?
            .permissions();
        perms.set_mode(0o600); // -rw------- (owner only)
        temp_file.as_file().set_permissions(perms)
            .context("Failed to set restrictive permissions on temp file")?;
    }

    // Write PDF data
    temp_file.write_all(pdf_bytes)
        .context("Failed to write PDF data to temp file")?;
    temp_file.flush()
        .context("Failed to flush temp file")?;

    // Parse using the temp file path
    let transactions = parse_hl_pdf(temp_file.path())?;

    // temp_file is automatically deleted when it goes out of scope
    Ok(transactions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Requires Python environment and sample PDF
    fn test_parse_spring_2024_pdf() {
        let transactions = parse_hl_pdf("Spring 2024 - Investment Report.pdf")
            .expect("Failed to parse Spring 2024 PDF");

        assert!(!transactions.is_empty(), "Should parse at least one transaction");

        // Check that we have various transaction types
        let buy_count = transactions.iter()
            .filter(|t| matches!(t.transaction_type, crate::overseer::enums::TransactionType::Buy))
            .count();

        assert!(buy_count > 0, "Should have Buy transactions");
    }

    #[test]
    #[ignore] // Requires Python environment and sample PDF
    fn test_parse_autumn_2025_pdf() {
        let transactions = parse_hl_pdf("Autumn 2025 - Investment Report.pdf")
            .expect("Failed to parse Autumn 2025 PDF");

        assert!(!transactions.is_empty(), "Should parse at least one transaction");
    }

    #[cfg(all(feature = "secure-temp-files", unix))]
    #[test]
    fn test_temp_file_permissions() {
        use std::os::unix::fs::PermissionsExt;
        use tempfile::NamedTempFile;

        let temp_file = NamedTempFile::new().unwrap();
        let mut perms = temp_file.as_file().metadata().unwrap().permissions();
        perms.set_mode(0o600);
        temp_file.as_file().set_permissions(perms).unwrap();

        let metadata = temp_file.as_file().metadata().unwrap();
        let mode = metadata.permissions().mode();

        assert_eq!(mode & 0o777, 0o600, "Permissions should be 0o600");
    }
}
