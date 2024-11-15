# tailf

A Rust library that provides an async file tailing functionality using the system's `tail` command, powered by `tokio::process::Command`.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tailf = "0.1.0"
```

## Usage

```rust
use tailf::tailf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read last 10 lines and follow new updates
    let num_lines = 10;
    let mut tailer = tailf("/var/log/syslog", Some(num_lines));

    while let Some(line) = tailer.next().await? {
        let line_str = String::from_utf8_lossy(&line);
        print!("{}", line_str);
        std::io::stdout().flush()?;
    }

    Ok(())
}
```

## Cleanup
The underlying `tail` process is automatically terminated when the `tailer` instance is dropped.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
