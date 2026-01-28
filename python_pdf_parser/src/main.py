import sys
import json
import re
from datetime import datetime
from decimal import Decimal
from enum import Enum
from dataclasses import dataclass, asdict
import pdfplumber
import pandas as pd
from pathlib import Path
from typing import List, Dict, Optional, Any


class TransactionType(str, Enum):
    """Transaction type enum matching Rust TransactionType."""
    BUY = "Buy"
    SELL = "Sell"
    DIVIDEND = "Dividend"
    DEPOSIT = "Deposit"
    WITHDRAWAL = "Withdrawal"
    FEE = "Fee"
    UNKNOWN = "Unknown"


@dataclass
class Stock:
    """Stock information matching Rust Stock struct."""
    ticker: str
    isin: str
    name: str


@dataclass
class Crypto:
    """Crypto information matching Rust Crypto struct."""
    symbol: str
    name: str


@dataclass
class Asset:
    """Asset type matching Rust Asset enum (tagged union)."""
    Stock: Optional[Stock] = None
    Crypto: Optional[Crypto] = None

    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary for JSON serialization."""
        if self.Stock:
            return {"Stock": asdict(self.Stock)}
        elif self.Crypto:
            return {"Crypto": asdict(self.Crypto)}
        return {}


@dataclass
class HistoricalTransaction:
    """Historical transaction matching Rust HistoricalTransaction struct."""
    asset: Asset
    date: str  # ISO 8601 format
    unit_price: str  # Decimal as string
    quantity: str  # Decimal as string
    total_value: str  # Decimal as string
    transaction_type: TransactionType

    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary for JSON serialization."""
        return {
            "asset": self.asset.to_dict(),
            "date": self.date,
            "unit_price": self.unit_price,
            "quantity": self.quantity,
            "total_value": self.total_value,
            "transaction_type": self.transaction_type.value
        }


class PDFTableParser:
    """Flexible PDF parser for extracting table data."""

    def __init__(self, pdf_path):
        self.pdf_path = Path(pdf_path)
        if not self.pdf_path.exists():
            raise FileNotFoundError(f"PDF file not found: {pdf_path}")

    def find_table_by_header(self, header_text, case_sensitive=False):
        """
        Find and extract tables that contain a specific header text.
        Also detects continuation tables on subsequent pages.

        Args:
            header_text: Text to search for in table headers
            case_sensitive: Whether to perform case-sensitive search

        Returns:
            List of dictionaries containing table data and metadata
        """
        results = []
        found_header_on_prev_page = False

        with pdfplumber.open(self.pdf_path) as pdf:
            for page_num, page in enumerate(pdf.pages, start=1):
                # Extract all tables from the page
                tables = page.extract_tables()

                # Also search for the header text in page text
                page_text = page.extract_text()
                if not case_sensitive:
                    contains_header = header_text.lower() in page_text.lower()
                    contains_continuation = 'continued from' in page_text.lower() or 'continued overleaf' in page_text.lower()
                else:
                    contains_header = header_text in page_text
                    contains_continuation = 'Continued from' in page_text or 'Continued overleaf' in page_text

                # Process tables if we found the header or this is a continuation
                if contains_header or (found_header_on_prev_page and contains_continuation):
                    # Process each table on this page
                    for table_idx, table in enumerate(tables):
                        if table and len(table) > 0:
                            # Check if this table is relevant
                            table_text = ' '.join([' '.join([str(cell) for cell in row if cell]) for row in table])

                            if not case_sensitive:
                                is_relevant = (header_text.lower() in table_text.lower() or
                                             (found_header_on_prev_page and contains_continuation))
                            else:
                                is_relevant = (header_text in table_text or
                                             (found_header_on_prev_page and contains_continuation))

                            if is_relevant:
                                # Clean the table data
                                cleaned_table = self._clean_table(table)

                                results.append({
                                    'page': page_num,
                                    'table_index': table_idx,
                                    'data': cleaned_table,
                                    'dataframe': self._to_dataframe(cleaned_table)
                                })

                    # Mark that we found the header on this page
                    if contains_header:
                        found_header_on_prev_page = True
                else:
                    # Reset if we don't find header or continuation
                    found_header_on_prev_page = False

        return results

    def _clean_table(self, table):
        """Clean table data by removing None values and empty rows."""
        cleaned = []
        for row in table:
            # Replace None with empty string and strip whitespace
            cleaned_row = [str(cell).strip() if cell is not None else '' for cell in row]
            # Only keep rows that have at least one non-empty cell
            if any(cell for cell in cleaned_row):
                cleaned.append(cleaned_row)
        return cleaned

    def _to_dataframe(self, table):
        """Convert table to pandas DataFrame with intelligent header detection."""
        if not table or len(table) < 2:
            return pd.DataFrame()

        # Find the actual header row (look for rows with expected column keywords)
        header_keywords = ['date', 'type', 'transaction', 'details', 'price', 'value', 'sedol', 'units']
        header_row_idx = 0

        # Search first 5 rows for the header
        for idx in range(min(5, len(table))):
            row_text = ' '.join([str(cell).lower() for cell in table[idx] if cell])
            # Check if this row contains multiple header keywords
            keyword_count = sum(1 for keyword in header_keywords if keyword in row_text)
            if keyword_count >= 3:  # If at least 3 keywords match, it's likely the header
                header_row_idx = idx
                break

        # Use detected row as headers
        headers = table[header_row_idx]
        data = table[header_row_idx + 1:]

        # Skip if no data rows
        if len(data) == 0:
            return pd.DataFrame()

        # Create DataFrame
        df = pd.DataFrame(data, columns=headers)

        # Clean up column names (remove extra whitespace)
        df.columns = [str(col).strip() for col in df.columns]

        return df

    def extract_all_tables(self):
        """Extract all tables from the PDF."""
        results = []

        with pdfplumber.open(self.pdf_path) as pdf:
            for page_num, page in enumerate(pdf.pages, start=1):
                tables = page.extract_tables()

                for table_idx, table in enumerate(tables):
                    if table and len(table) > 0:
                        cleaned_table = self._clean_table(table)
                        results.append({
                            'page': page_num,
                            'table_index': table_idx,
                            'data': cleaned_table,
                            'dataframe': self._to_dataframe(cleaned_table)
                        })

        return results

    def merge_multi_page_tables(self, results):
        """
        Merge tables from multiple pages that have the same structure.
        Assumes tables are continuation of each other if on consecutive pages.
        """
        if len(results) <= 1:
            return results

        # Filter out empty tables first (tables with no data rows)
        non_empty_results = [r for r in results if not r['dataframe'].empty and len(r['dataframe']) > 0]

        if len(non_empty_results) <= 1:
            return non_empty_results

        # Group tables by whether they appear to be the same structure
        merged_results = []
        current_group = [non_empty_results[0]]

        for i in range(1, len(non_empty_results)):
            prev_result = non_empty_results[i - 1]
            curr_result = non_empty_results[i]

            prev_df = prev_result['dataframe']
            curr_df = curr_result['dataframe']

            is_continuation = False

            # Check if pages are consecutive or close (within 2 pages)
            if curr_result['page'] - prev_result['page'] <= 2:
                # Check if column structure is similar
                # Normalize: lowercase, strip, collapse whitespace, remove newlines
                prev_cols = set([' '.join(str(col).lower().strip().split()) for col in prev_df.columns])
                curr_cols = set([' '.join(str(col).lower().strip().split()) for col in curr_df.columns])

                # Debug: print columns for troubleshooting
                if len(prev_cols) > 0 and len(curr_cols) > 0:
                    overlap = len(prev_cols.intersection(curr_cols))
                    max_cols = max(len(prev_cols), len(curr_cols))
                    overlap_percent = overlap / max_cols

                    # Print debug info to stderr
                    print(f"   Comparing Page {prev_result['page']} with Page {curr_result['page']}:", file=sys.stderr)
                    print(f"   Overlap: {overlap}/{max_cols} columns ({overlap_percent:.1%})", file=sys.stderr)
                    if overlap_percent <= 0.5:
                        print(f"   Prev cols: {prev_cols}", file=sys.stderr)
                        print(f"   Curr cols: {curr_cols}", file=sys.stderr)

                    # If columns overlap significantly (>50%), it's likely a continuation
                    if overlap_percent > 0.5:
                        is_continuation = True

            if is_continuation:
                current_group.append(curr_result)
            else:
                # Save current group and start new one
                if current_group:
                    merged_results.append(self._merge_table_group(current_group))
                current_group = [curr_result]

        # Don't forget the last group
        if current_group:
            merged_results.append(self._merge_table_group(current_group))

        return merged_results

    def _merge_table_group(self, table_group):
        """Merge a group of tables that are continuations of each other."""
        if len(table_group) == 1:
            return table_group[0]

        # Start with first table's metadata
        merged = {
            'page': table_group[0]['page'],
            'page_range': f"{table_group[0]['page']}-{table_group[-1]['page']}",
            'table_index': table_group[0]['table_index'],
            'data': [],
            'dataframe': pd.DataFrame()
        }

        # Merge dataframes
        dfs_to_merge = []
        for result in table_group:
            df = result['dataframe'].copy()
            if not df.empty:
                # Handle duplicate column names by making them unique
                cols = df.columns.tolist()
                seen = {}
                new_cols = []
                for col in cols:
                    col_str = str(col)
                    if col_str in seen:
                        seen[col_str] += 1
                        new_cols.append(f"{col_str}_{seen[col_str]}")
                    else:
                        seen[col_str] = 0
                        new_cols.append(col_str)
                df.columns = new_cols

                dfs_to_merge.append(df)

        if dfs_to_merge:
            # Ensure all dataframes have the same columns (use first as reference)
            reference_cols = dfs_to_merge[0].columns.tolist()

            aligned_dfs = []
            for df in dfs_to_merge:
                # Reindex to match reference columns
                df_aligned = df.reindex(columns=reference_cols, fill_value='')
                aligned_dfs.append(df_aligned)

            # Concatenate all dataframes, ignoring the index
            merged_df = pd.concat(aligned_dfs, ignore_index=True)

            # Clean up the merged dataframe
            if len(merged_df) > 0 and len(merged_df.columns) > 0:
                # Remove duplicate header rows that might have been included as data
                first_col = merged_df.columns[0]
                merged_df = merged_df[merged_df[first_col].astype(str).str.lower() != str(first_col).lower()]

                # Remove footer rows and other non-transaction data
                # Look for common footer/separator keywords
                footer_keywords = [
                    'the capital account shows',
                    'the income account shows',
                    'venue of execution',
                    'income account transactions',
                    'ncome account transactions',  # Sometimes 'I' is missing
                    'your instruction for income',
                    'continued from',
                    'code description',
                    'london stock exchange',
                    'optiver',
                    'off exchange'
                ]

                # Filter out rows that contain footer text in any column
                import re

                def is_valid_row(row):
                    # Check if entire row is footer-like
                    row_text = ' '.join([str(cell).lower() for cell in row if cell and str(cell).strip()])

                    # If row contains footer keywords, exclude it
                    for keyword in footer_keywords:
                        if keyword in row_text:
                            return False

                    # Validate first column looks like a date (DD/MM/YYYY format)
                    first_cell = str(row.iloc[0]).strip()
                    if not first_cell or first_cell == '' or first_cell == 'nan':
                        return False

                    # Check if first cell looks like a date (contains / and numbers)
                    date_pattern = r'\d{2}/\d{2}/\d{4}'
                    if not re.match(date_pattern, first_cell):
                        return False

                    # Check for INCOME ACCOUNT specific patterns (gross loyalty, etc.)
                    if 'gross loyalty' in row_text or 'jpmorgan' in row_text:
                        return False

                    # For CAPITAL ACCOUNT: validate column structure
                    # Column 1: Trade Type (Bought/Sold or empty)
                    # Column 2: SEDOL
                    # Column 3: Venue Code
                    # Column 4: Transaction Details
                    if len(row) >= 5:
                        trade_type = str(row.iloc[1]).strip().lower()
                        sedol = str(row.iloc[2]).strip()
                        venue = str(row.iloc[3]).strip()
                        transaction_details = str(row.iloc[4]).strip().lower()

                        # INCOME ACCOUNT has description in column 1, not column 4
                        # If column 1 has long text and column 4 is empty, it's INCOME ACCOUNT
                        col1_text = str(row.iloc[1]).strip()
                        if len(col1_text) > 20 and (transaction_details == '' or transaction_details == 'nan'):
                            return False

                        # Valid non-trade transactions in CAPITAL ACCOUNT
                        valid_non_trade = ['interest', 'bonus', 'fee', 'subscription', 'management', 're-investment', 'balance brought forward']

                        # If no trade type, must have transaction details in column 4
                        if trade_type == '' or trade_type == 'nan':
                            # Must have details in the right column (column 4)
                            if transaction_details == '' or transaction_details == 'nan':
                                return False

                            # Check for valid non-trade keywords in column 4
                            has_valid_keyword = any(keyword in transaction_details for keyword in valid_non_trade)
                            if not has_valid_keyword:
                                return False

                    return True

                # Apply filter
                mask = merged_df.apply(is_valid_row, axis=1)
                merged_df = merged_df[mask]

            merged['dataframe'] = merged_df

        return merged


class HLTransactionParser:
    """Parser for Hargreaves Lansdown Capital Account Transactions."""

    @staticmethod
    def parse_financial_value(value_str: str) -> Optional[Decimal]:
        """Parse financial string to Decimal, handling parentheses and formatting."""
        if not value_str or value_str.strip() == '':
            return None

        # Remove whitespace, £, and commas
        cleaned = value_str.strip().replace('£', '').replace(',', '').replace(' ', '')

        # Handle parentheses (negative values)
        is_negative = False
        if cleaned.startswith('(') and cleaned.endswith(')'):
            is_negative = True
            cleaned = cleaned[1:-1]

        try:
            value = Decimal(cleaned)
            return -value if is_negative else value
        except:
            return None

    @staticmethod
    def parse_date(date_str: str) -> Optional[str]:
        """Parse date string to ISO format."""
        if not date_str or date_str.strip() == '':
            return None

        # Try common date formats
        formats = ['%d/%m/%Y', '%Y-%m-%d', '%d-%m-%Y']
        for fmt in formats:
            try:
                dt = datetime.strptime(date_str.strip(), fmt)
                # Return ISO 8601 format with UTC timezone
                return dt.strftime('%Y-%m-%dT%H:%M:%SZ')
            except:
                continue
        return None

    @staticmethod
    def map_transaction_type(trade_type: str, details: str = "") -> TransactionType:
        """Map HL trade type to TransactionType enum."""
        # Normalize inputs
        trade_type_lower = trade_type.strip().lower() if trade_type else ""
        details_lower = details.lower() if details else ""

        # Check both trade_type and details for patterns
        # Buy/Sell transactions
        if 'bought' in trade_type_lower or 'buy' in trade_type_lower:
            return TransactionType.BUY
        elif 'sold' in trade_type_lower or 'sell' in trade_type_lower:
            return TransactionType.SELL

        # Income transactions (dividends, interest, bonuses)
        elif 'dividend' in trade_type_lower or 'dividend' in details_lower:
            return TransactionType.DIVIDEND
        elif 'income' in details_lower and 're-investment' in details_lower:
            return TransactionType.DIVIDEND

        # Deposits (interest, bonuses, deposits)
        elif 'interest' in details_lower:
            return TransactionType.DEPOSIT
        elif 'bonus' in details_lower:
            return TransactionType.DEPOSIT
        elif 'deposit' in trade_type_lower or 'deposit' in details_lower:
            return TransactionType.DEPOSIT

        # Withdrawals
        elif 'withdrawal' in trade_type_lower or 'withdraw' in trade_type_lower:
            return TransactionType.WITHDRAWAL

        # Fees and charges
        elif 'fee' in trade_type_lower or 'fee' in details_lower:
            return TransactionType.FEE
        elif 'charge' in trade_type_lower or 'charge' in details_lower:
            return TransactionType.FEE
        elif 'management fee' in details_lower:
            return TransactionType.FEE

        # Special cases - skip these by returning Unknown (caller can filter)
        elif 'balance brought forward' in details_lower:
            return TransactionType.UNKNOWN  # Opening balance, not a real transaction

        else:
            return TransactionType.UNKNOWN

    @staticmethod
    def extract_stock_info(transaction_details: str, sedol: str = "") -> Stock:
        """Extract stock information from transaction details."""
        # Clean up the transaction details
        details = transaction_details.strip()

        # Extract stock name (usually before @ symbol or numeric price)
        name_match = re.match(r'^(.+?)(?:\s+@\s+\d+|\s+ORD|\s+USD|\s+GBP|\s+plc)', details, re.IGNORECASE)
        if name_match:
            name = name_match.group(1).strip()
        else:
            name = details

        # Try to extract ticker from details if present
        ticker_match = re.search(r'\b([A-Z]{3,5})\b', details)
        ticker = ticker_match.group(1) if ticker_match else sedol

        return Stock(
            ticker=ticker if ticker else "UNKNOWN",
            isin=sedol if sedol else "UNKNOWN",
            name=name if name else "Unknown Stock"
        )

    def parse_capital_account_transactions(self, df: pd.DataFrame) -> List[HistoricalTransaction]:
        """Parse Capital Account Transactions table into HistoricalTransaction objects."""
        transactions = []

        # Check if headers are already properly set (by _to_dataframe)
        column_names_lower = [str(col).lower() for col in df.columns]
        headers_already_set = any('date' in col for col in column_names_lower) and \
                             any('type' in col or 'sedol' in col for col in column_names_lower)

        if not headers_already_set:
            # Find the header row index
            header_row_idx = None
            for idx, row in df.iterrows():
                row_str = ' '.join([str(cell).lower() for cell in row])
                if 'transaction' in row_str and 'date' in row_str:
                    header_row_idx = idx
                    break

            if header_row_idx is None:
                # Try to use first row as header
                df.columns = df.iloc[0]
                df = df[1:]
            elif header_row_idx > 0:
                df.columns = df.iloc[header_row_idx]
                df = df[header_row_idx + 1:]

            # Reset index
            df = df.reset_index(drop=True)

        # Clean column names
        df.columns = [str(col).strip().replace('\n', ' ') for col in df.columns]

        # Find relevant columns (case-insensitive)
        col_mapping = {}
        for col in df.columns:
            col_lower = col.lower()
            if 'transaction' in col_lower and 'date' in col_lower:
                col_mapping['date'] = col
            elif 'trade' in col_lower and 'type' in col_lower:
                col_mapping['trade_type'] = col
            elif 'sedol' in col_lower:
                col_mapping['sedol'] = col
            elif 'transaction' in col_lower and 'details' in col_lower:
                col_mapping['details'] = col
            elif 'units' in col_lower:
                col_mapping['units'] = col
            elif 'unit' in col_lower and 'price' in col_lower:
                col_mapping['unit_price'] = col
            elif 'value' in col_lower and 'balance' not in col_lower:
                col_mapping['value'] = col

        # Parse each row
        for idx, row in df.iterrows():
            try:
                # Skip empty rows
                if row.isna().all() or all(str(cell).strip() == '' for cell in row):
                    continue

                # Extract date
                date_str = str(row.get(col_mapping.get('date', ''), '')).strip()
                date = self.parse_date(date_str)

                # Skip rows without valid date
                if not date:
                    continue

                # Extract other fields
                trade_type = str(row.get(col_mapping.get('trade_type', ''), '')).strip()
                sedol = str(row.get(col_mapping.get('sedol', ''), '')).strip()
                details = str(row.get(col_mapping.get('details', ''), '')).strip()
                units_str = str(row.get(col_mapping.get('units', ''), '')).strip()
                unit_price_str = str(row.get(col_mapping.get('unit_price', ''), '')).strip()
                value_str = str(row.get(col_mapping.get('value', ''), '')).strip()

                # Parse numeric values
                quantity = self.parse_financial_value(units_str)
                unit_price = self.parse_financial_value(unit_price_str)
                total_value = self.parse_financial_value(value_str)

                # Handle unit price in pence (convert to pounds)
                if unit_price and unit_price > 100:
                    unit_price = unit_price / 100

                # Map transaction type
                transaction_type = self.map_transaction_type(trade_type, details)

                # Extract stock info
                if details and details != '' and transaction_type in [TransactionType.BUY, TransactionType.SELL]:
                    stock_info = self.extract_stock_info(details, sedol)
                    asset = Asset(Stock=stock_info)
                else:
                    # For non-stock transactions, use generic info
                    asset = Asset(Stock=Stock(
                        ticker="CASH",
                        isin="",
                        name=details if details else transaction_type.value
                    ))

                # Create transaction object
                transaction = HistoricalTransaction(
                    asset=asset,
                    date=date,
                    unit_price=str(unit_price) if unit_price else "0",
                    quantity=str(quantity) if quantity else "0",
                    total_value=str(total_value) if total_value else "0",
                    transaction_type=transaction_type
                )

                transactions.append(transaction)

            except Exception as e:
                # Skip rows that fail to parse
                print(f"Warning: Failed to parse row {idx}: {e}", file=sys.stderr)
                continue

        return transactions


def main():
    """Main entry point for the PDF parser."""
    if len(sys.argv) < 2:
        print("Usage: python main.py <pdf_file_path> [options]")
        print("\nOptions:")
        print("  --json                    Output as JSON for Rust integration")
        print("  --table <header_text>     Search for specific table by header")
        print("\nExamples:")
        print("  python main.py document.pdf")
        print("  python main.py document.pdf --table 'CAPITAL ACCOUNT TRANSACTIONS'")
        print("  python main.py document.pdf --json --table 'CAPITAL ACCOUNT TRANSACTIONS'")
        sys.exit(1)

    pdf_path = sys.argv[1]
    json_output = '--json' in sys.argv
    table_header = None

    # Check for --table argument
    if '--table' in sys.argv:
        table_idx = sys.argv.index('--table')
        if table_idx + 1 < len(sys.argv):
            table_header = sys.argv[table_idx + 1]

    try:
        parser = PDFTableParser(pdf_path)

        # If searching for specific table
        if table_header:
            if not json_output:
                print(f"Searching for tables containing: '{table_header}'\n", file=sys.stderr)

            results = parser.find_table_by_header(table_header)

            if not results:
                if not json_output:
                    print(f"No tables found containing '{table_header}'", file=sys.stderr)
                else:
                    print(json.dumps({"transactions": []}))
                return

            # Merge tables that span multiple pages
            if not json_output:
                print(f"Found {len(results)} table(s) across pages, merging...\n", file=sys.stderr)
            results = parser.merge_multi_page_tables(results)
            if not json_output:
                print(f"After merging: {len(results)} table(s)\n", file=sys.stderr)

            # Check if this is Capital Account Transactions
            is_capital_account = 'capital account' in table_header.lower()

            if json_output and is_capital_account:
                # Parse transactions and output JSON
                transaction_parser = HLTransactionParser()
                all_transactions = []

                for result in results:
                    df = result['dataframe']
                    if not df.empty:
                        transactions = transaction_parser.parse_capital_account_transactions(df)
                        all_transactions.extend(transactions)

                # Convert to dict and output JSON
                output = {
                    "transactions": [t.to_dict() for t in all_transactions]
                }
                print(json.dumps(output, indent=2))
            elif json_output:
                # Generic JSON output for other tables
                output_data = []
                for result in results:
                    df = result['dataframe']
                    if not df.empty:
                        output_data.append({
                            "page": result['page'],
                            "table_index": result['table_index'],
                            "data": df.to_dict(orient='records')
                        })
                print(json.dumps(output_data, indent=2))
            else:
                # Human-readable output
                print(f"Found {len(results)} table(s):\n", file=sys.stderr)
                for result in results:
                    print(f"Page {result['page']}, Table {result['table_index']}:")
                    print("-" * 80)

                    df = result['dataframe']
                    if not df.empty:
                        print(df.to_string(index=False))
                    else:
                        for row in result['data']:
                            print(' | '.join(row))

                    print("\n")
        else:
            # Extract all tables
            if not json_output:
                print("Extracting all tables from PDF...\n", file=sys.stderr)

            results = parser.extract_all_tables()

            if not results:
                if not json_output:
                    print("No tables found in the PDF", file=sys.stderr)
                else:
                    print(json.dumps({"tables": []}))
                return

            if json_output:
                # Generic JSON output
                output_data = []
                for result in results:
                    df = result['dataframe']
                    if not df.empty:
                        output_data.append({
                            "page": result['page'],
                            "table_index": result['table_index'],
                            "data": df.to_dict(orient='records')
                        })
                print(json.dumps(output_data, indent=2))
            else:
                # Human-readable output
                print(f"Found {len(results)} table(s):\n", file=sys.stderr)
                for result in results:
                    print(f"Page {result['page']}, Table {result['table_index']}:")
                    print("-" * 80)

                    df = result['dataframe']
                    if not df.empty:
                        print(df.to_string(index=False))
                    else:
                        for row in result['data']:
                            print(' | '.join(row))

                    print("\n")

    except FileNotFoundError as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"Error parsing PDF: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc(file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
