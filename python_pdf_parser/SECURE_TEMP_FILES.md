# Secure Temporary File Handling

When processing sensitive financial PDFs, proper temporary file security is critical.

## The Problem with `/tmp`

```rust
// ⚠️ INSECURE - Don't do this!
std::fs::write("/tmp/report.pdf", bytes)?;
```

Issues:
- Default permissions may allow other users to read
- Predictable filenames can be guessed
- Files may persist after crashes
- Visible to system monitoring/backups

## Secure Solution

### Using `tempfile` Crate (RECOMMENDED)

Add to your `Cargo.toml`:
```toml
[dependencies]
tempfile = "3.8"
```

### Secure Temporary File Creation

```rust
use tempfile::NamedTempFile;
use std::io::Write;
use std::process::Command;

fn parse_pdf_securely(pdf_bytes: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    // Create secure temporary file
    // - Random filename (prevents guessing)
    // - Automatic cleanup when dropped
    // - Atomic creation (prevents race conditions)
    let mut temp_file = NamedTempFile::new()?;

    // Set restrictive permissions: owner read/write only (0o600)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = temp_file.as_file().metadata()?.permissions();
        perms.set_mode(0o600);  // -rw------- (owner only)
        temp_file.as_file().set_permissions(perms)?;
    }

    // Write PDF data
    temp_file.write_all(pdf_bytes)?;
    temp_file.flush()?;  // Ensure data is written

    // Get the path
    let temp_path = temp_file.path();

    // Parse PDF
    let output = Command::new("python")
        .arg("src/main.py")
        .arg(temp_path)
        .arg("--json")
        .arg("--table")
        .arg("CAPITAL ACCOUNT TRANSACTIONS")
        .output()?;

    // Convert output to string
    let json_str = String::from_utf8(output.stdout)?;

    // File is automatically deleted when temp_file goes out of scope
    Ok(json_str)
}
```

### Verify Permissions

```rust
use std::os::unix::fs::PermissionsExt;

// Check file permissions
let metadata = temp_file.as_file().metadata()?;
let mode = metadata.permissions().mode();
assert_eq!(mode & 0o777, 0o600, "File permissions are not restrictive enough");
```

### Custom Secure Directory

If you want more control over the temporary directory location:

```rust
use tempfile::Builder;
use std::path::PathBuf;

fn get_secure_temp_dir() -> Result<PathBuf, std::io::Error> {
    // Option 1: Use user's home directory
    let home = std::env::var("HOME").expect("HOME not set");
    let secure_dir = PathBuf::from(home).join(".secure_temp");

    // Create directory with restrictive permissions
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        use std::fs::DirBuilder;

        DirBuilder::new()
            .mode(0o700)  // drwx------ (owner only)
            .recursive(true)
            .create(&secure_dir)?;
    }

    #[cfg(not(unix))]
    {
        std::fs::create_dir_all(&secure_dir)?;
    }

    Ok(secure_dir)
}

fn parse_pdf_with_custom_dir(pdf_bytes: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let secure_dir = get_secure_temp_dir()?;

    // Create temp file in secure directory
    let mut temp_file = Builder::new()
        .prefix("pdf_")
        .suffix(".pdf")
        .tempfile_in(secure_dir)?;

    // Set permissions
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = temp_file.as_file().metadata()?.permissions();
        perms.set_mode(0o600);
        temp_file.as_file().set_permissions(perms)?;
    }

    temp_file.write_all(pdf_bytes)?;
    temp_file.flush()?;

    // Parse...
    let output = Command::new("python")
        .arg("src/main.py")
        .arg(temp_file.path())
        .arg("--json")
        .arg("--table")
        .arg("CAPITAL ACCOUNT TRANSACTIONS")
        .output()?;

    Ok(String::from_utf8(output.stdout)?)
}
```

## Security Checklist

- ✅ **Permissions**: File is 0o600 (owner read/write only)
- ✅ **Directory**: Secure location with 0o700 permissions
- ✅ **Filename**: Random, unpredictable (via `tempfile` crate)
- ✅ **Creation**: Atomic (no race conditions)
- ✅ **Cleanup**: Automatic deletion via RAII
- ✅ **No symlinks**: `tempfile` prevents symlink attacks
- ✅ **Error handling**: Proper cleanup on errors

## Windows Considerations

On Windows, use:

```rust
#[cfg(windows)]
{
    use std::os::windows::fs::MetadataExt;
    // Windows has different permission model
    // Files in user's temp directory are automatically restricted
    // Consider using Windows ACLs for fine-grained control
}
```

## Additional Security Measures

### 1. Secure File Deletion (Optional)

For extra paranoia, overwrite before deletion:

```rust
use std::io::{Seek, SeekFrom, Write};

fn secure_delete(file: &mut NamedTempFile) -> std::io::Result<()> {
    let len = file.as_file().metadata()?.len();
    file.seek(SeekFrom::Start(0))?;

    // Overwrite with zeros
    let zeros = vec![0u8; len as usize];
    file.write_all(&zeros)?;
    file.flush()?;

    // File will be deleted when dropped
    Ok(())
}
```

### 2. Memory Locking (Advanced)

Prevent swapping to disk:

```rust
// Requires root/capabilities on Linux
use libc::{mlock, munlock};

unsafe {
    mlock(pdf_bytes.as_ptr() as *const _, pdf_bytes.len());
}
// ... process data
unsafe {
    munlock(pdf_bytes.as_ptr() as *const _, pdf_bytes.len());
}
```

### 3. Environment Variable for Temp Dir

```rust
// Set secure temp directory via environment
std::env::set_var("TMPDIR", "/home/user/.secure_temp");

// tempfile will respect TMPDIR
let temp_file = NamedTempFile::new()?;
```

## Complete Example

```rust
use tempfile::NamedTempFile;
use std::io::Write;
use std::process::Command;

fn process_financial_pdf(pdf_bytes: &[u8]) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // Create secure temporary file
    let mut temp_file = NamedTempFile::new()?;

    // Secure permissions (Unix only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = temp_file.as_file().metadata()?.permissions();
        perms.set_mode(0o600);
        temp_file.as_file().set_permissions(perms)?;
    }

    // Write data
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

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Parser failed: {}", stderr).into());
    }

    // Parse JSON
    let json_str = String::from_utf8(output.stdout)?;
    let data: serde_json::Value = serde_json::from_str(&json_str)?;

    // temp_file is automatically deleted here
    Ok(data)
}
```

## Testing Security

```rust
#[test]
fn test_temp_file_permissions() {
    use std::os::unix::fs::PermissionsExt;

    let temp_file = NamedTempFile::new().unwrap();
    let mut perms = temp_file.as_file().metadata().unwrap().permissions();
    perms.set_mode(0o600);
    temp_file.as_file().set_permissions(perms).unwrap();

    let metadata = temp_file.as_file().metadata().unwrap();
    let mode = metadata.permissions().mode();

    assert_eq!(mode & 0o777, 0o600, "Permissions should be 0o600");
}
```

## Summary

**Always use the `tempfile` crate** for handling temporary files containing sensitive data:

1. Automatic cleanup (RAII)
2. Secure random filenames
3. Atomic creation
4. Cross-platform support
5. Set permissions to 0o600 (Unix)
6. Consider custom secure directory for extra control

**Never**:
- Use predictable filenames
- Leave files with world-readable permissions
- Forget to clean up temporary files
- Store sensitive data in shared temporary directories
