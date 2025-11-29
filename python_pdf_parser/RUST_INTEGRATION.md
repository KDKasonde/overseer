# Rust Integration Guide

This Python PDF parser outputs JSON that matches the Rust `HistoricalTransaction` struct from the overseer project.

## Python Data Structures

The parser uses Python enums and dataclasses that mirror the Rust types:

### TransactionType Enum
```python
class TransactionType(str, Enum):
    BUY = "Buy"
    SELL = "Sell"
    DIVIDEND = "Dividend"
    DEPOSIT = "Deposit"
    WITHDRAWAL = "Withdrawal"
    FEE = "Fee"
    UNKNOWN = "Unknown"
```

Maps to Rust:
```rust
pub enum TransactionType {
    Buy,
    Sell,
    Dividend,
    Deposit,
    Withdrawal,
    Fee,
    Unknown,
}
```

### Stock Struct
```python
@dataclass
class Stock:
    ticker: str
    isin: str
    name: str
```

Maps to Rust:
```rust
pub struct Stock {
    pub ticker: String,
    pub isin: String,
    pub name: String,
}
```

### Asset Enum (Tagged Union)
```python
@dataclass
class Asset:
    Stock: Optional[Stock] = None
    Crypto: Optional[Crypto] = None
```

Maps to Rust:
```rust
pub enum Asset {
    Stock(Stock),
    Crypto(Crypto),
}
```

### HistoricalTransaction Struct
```python
@dataclass
class HistoricalTransaction:
    asset: Asset
    date: str  # ISO 8601 format
    unit_price: str  # Decimal as string
    quantity: str  # Decimal as string
    total_value: str  # Decimal as string
    transaction_type: TransactionType
```

Maps to Rust:
```rust
pub struct HistoricalTransaction {
    pub asset: Asset,
    pub date: DateTime<Utc>,
    pub unit_price: Decimal,
    pub quantity: Decimal,
    pub total_value: Decimal,
    pub transaction_type: TransactionType,
}
```

## Usage

### Command Line

Extract Capital Account Transactions as JSON:

```bash
python src/main.py "document.pdf" --json --table "CAPITAL ACCOUNT TRANSACTIONS"
```

This outputs JSON to stdout:

```json
{
  "transactions": [
    {
      "asset": {
        "Stock": {
          "ticker": "BKDRYJ4",
          "isin": "BKDRYJ4",
          "name": "Airtel Africa plc"
        }
      },
      "date": "2025-08-05T00:00:00Z",
      "unit_price": "2.1275",
      "quantity": "141",
      "total_value": "-313.44",
      "transaction_type": "Buy"
    }
  ]
}
```

### From Rust

Call the Python parser from Rust using `pyo3` or by spawning a subprocess:

#### Option 1: Using subprocess with file path (Simple)

```rust
use std::process::Command;
use serde_json::Value;

fn parse_pdf_transactions(pdf_path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let output = Command::new("python")
        .arg("src/main.py")
        .arg(pdf_path)
        .arg("--json")
        .arg("--table")
        .arg("CAPITAL ACCOUNT TRANSACTIONS")
        .output()?;

    let json_str = String::from_utf8(output.stdout)?;
    let data: Value = serde_json::from_str(&json_str)?;
    Ok(data)
}
```

#### Option 1b: Using subprocess with secure temporary file (RECOMMENDED for HTTP downloads)

**⚠️ SECURITY**: If processing sensitive financial data from HTTP/network sources, use secure temporary files. See [SECURE_TEMP_FILES.md](SECURE_TEMP_FILES.md) for complete security guidance.

```rust
use tempfile::NamedTempFile;
use std::io::Write;
use std::process::Command;

fn parse_pdf_from_bytes(pdf_bytes: &[u8]) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // Create secure temporary file with restrictive permissions
    let mut temp_file = NamedTempFile::new()?;

    // Set owner-only permissions (0o600)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = temp_file.as_file().metadata()?.permissions();
        perms.set_mode(0o600);  // -rw------- (owner only)
        temp_file.as_file().set_permissions(perms)?;
    }

    // Write PDF data and flush
    temp_file.write_all(pdf_bytes)?;
    temp_file.flush()?;

    // Parse PDF
    let output = Command::new("python")
        .arg("src/main.py")
        .arg(temp_file.path())
        .arg("--json")
        .arg("--table")
        .arg("CAPITAL ACCOUNT TRANSACTIONS")
        .output()?;

    let json_str = String::from_utf8(output.stdout)?;
    let data: serde_json::Value = serde_json::from_str(&json_str)?;

    // temp_file is automatically deleted here
    Ok(data)
}
```

#### Option 2: Using PyO3 (Advanced)

```rust
use pyo3::prelude::*;
use pyo3::types::PyModule;

fn parse_pdf_transactions(pdf_path: &str) -> PyResult<String> {
    Python::with_gil(|py| {
        let parser_module = PyModule::from_code(
            py,
            include_str!("../python_parser/src/main.py"),
            "main.py",
            "main",
        )?;

        let parser_class = parser_module.getattr("PDFTableParser")?;
        let parser = parser_class.call1((pdf_path,))?;

        let results = parser.call_method1("find_table_by_header", ("CAPITAL ACCOUNT TRANSACTIONS",))?;

        // Process results...
        Ok("json_output".to_string())
    })
}
```

## Deserializing in Rust

The JSON can be deserialized directly into your Rust structs using `serde`:

```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Serialize, Deserialize, Debug)]
struct TransactionsResponse {
    transactions: Vec<HistoricalTransaction>,
}

// Then deserialize:
let response: TransactionsResponse = serde_json::from_str(&json_output)?;
```

## Date Format

Dates are output in ISO 8601 format with UTC timezone:
- Python: `"2025-08-05T00:00:00Z"`
- Rust: Deserializes to `DateTime<Utc>`

## Decimal Values

Numeric values (prices, quantities, totals) are serialized as strings to preserve precision:
- Python: Uses `Decimal` type, serializes to string
- Rust: Deserialize as `String` then parse to `Decimal` using `rust_decimal` crate

## Error Handling

- Invalid rows are logged to stderr and skipped
- Rows without valid dates are skipped
- Failed numeric parsing defaults to "0"
- Unknown transaction types default to "Unknown"

## Testing

Test the JSON output:

```bash
python test_json_output.py
```

This will parse the sample PDF and display the JSON output.

## Integration Example

Complete Rust integration example:

```rust
use serde::{Deserialize, Serialize};
use std::process::Command;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
struct TransactionsResponse {
    transactions: Vec<HistoricalTransaction>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Call Python parser
    let output = Command::new("python")
        .arg("src/main.py")
        .arg("Autumn 2025 - Investment Report.pdf")
        .arg("--json")
        .arg("--table")
        .arg("CAPITAL ACCOUNT TRANSACTIONS")
        .output()?;

    // Parse JSON
    let json_str = String::from_utf8(output.stdout)?;
    let response: TransactionsResponse = serde_json::from_str(&json_str)?;

    // Use the transactions
    for tx in response.transactions {
        println!("Transaction: {:?}", tx);
    }

    Ok(())
}
```

## Notes

- The parser is flexible and works with various PDF formats, not just LISA accounts
- All string comparisons are case-insensitive
- The parser handles various date formats and financial number formats
- Parentheses in financial values are treated as negative numbers
