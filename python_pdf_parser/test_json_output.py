#!/usr/bin/env python3
"""Test script to verify JSON output for Rust integration."""

import sys
sys.path.insert(0, 'src')

from main import PDFTableParser, HLTransactionParser
import json

# Test with the Autumn 2025 PDF
pdf_path = "Autumn 2025 - Investment Report.pdf"

print("Testing PDF parser with JSON output...\n")

try:
    parser = PDFTableParser(pdf_path)
    results = parser.find_table_by_header("CAPITAL ACCOUNT TRANSACTIONS")

    if not results:
        print("No tables found")
        sys.exit(1)

    print(f"Found {len(results)} table(s)\n")

    transaction_parser = HLTransactionParser()
    all_transactions = []

    for result in results:
        df = result['dataframe']
        if not df.empty:
            print(f"Processing table from page {result['page']}...")
            transactions = transaction_parser.parse_capital_account_transactions(df)
            all_transactions.extend(transactions)
            print(f"Parsed {len(transactions)} transactions\n")

    # Convert to JSON
    output = {
        "transactions": [t.to_dict() for t in all_transactions]
    }

    print("JSON Output:")
    print("=" * 80)
    print(json.dumps(output, indent=2))
    print("=" * 80)
    print(f"\nTotal transactions parsed: {len(all_transactions)}")

except Exception as e:
    print(f"Error: {e}")
    import traceback
    traceback.print_exc()
    sys.exit(1)
