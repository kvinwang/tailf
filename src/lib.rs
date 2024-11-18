use std::io;
use std::path::Path;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::process::{Child, Command};

/// A tailer for a file.
pub struct Tailer {
    process: Option<Child>,
    reader: BufReader<tokio::process::ChildStdout>,
    max_chunk_size: u64,
}

impl Drop for Tailer {
    fn drop(&mut self) {
        // Cleanup: kill the tail process when dropped
        if let Some(mut process) = self.process.take() {
            let _ = process.start_kill();
            // Spawn a new task to wait for the process to exit
            tokio::spawn(async move {
                let _ = process.wait().await;
            });
        }
    }
}

impl Tailer {
    pub async fn next(&mut self) -> io::Result<Option<Vec<u8>>> {
        let mut line = Vec::new();
        match (&mut self.reader)
            .take(self.max_chunk_size)
            .read_until(b'\n', &mut line)
            .await
        {
            Ok(0) => Ok(None), // EOF
            Ok(_) => Ok(Some(line)),
            Err(e) => Err(e),
        }
    }
}

/// Options for the tailer.
#[derive(Debug, Clone, bon::Builder)]
pub struct Options {
    /// The number of lines to read from the end of the file.
    #[builder(required, default = None)]
    num_lines: Option<usize>,
    /// The maximum size of a chunk (line) to read from the file.
    #[builder(default = 1024 * 8)]
    max_chunk_size: u64,
    /// Whether to follow the file.
    #[builder(default = false)]
    follow: bool,
}

impl Options {
    /// Create a new tailer for a file.
    pub fn tail(&self, filename: impl AsRef<Path>) -> io::Result<Tailer> {
        let mut cmd = Command::new("tail");

        // Add -n flag if num_lines is provided
        if let Some(num_lines) = self.num_lines {
            cmd.arg("-n").arg(num_lines.to_string());
        }

        if self.follow {
            cmd.arg("-f");
        }

        cmd.arg(filename.as_ref());

        // Configure stdio
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::null());

        // Spawn the process
        let mut child = cmd.spawn()?;

        // Take stdout, leaving None in its place
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "child stdout handle missing"))?;
        let reader = BufReader::new(stdout);

        Ok(Tailer {
            process: Some(child),
            reader,
            max_chunk_size: self.max_chunk_size,
        })
    }
}

/// Create a new tailer for a file.
///
/// # Arguments
///
/// * `filename` - The path to the file to tail
/// * `num_lines` - The number of lines to read from the end of the file
pub fn tailf(filename: impl AsRef<Path>, num_lines: Option<usize>) -> io::Result<Tailer> {
    Options::builder()
        .follow(true)
        .num_lines(num_lines)
        .build()
        .tail(filename)
}
