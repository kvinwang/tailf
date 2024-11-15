use tailf::tailf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = std::env::args().nth(1).expect("Usage: tailf <filename>");
    let mut tailer = tailf(&filename, None)?;

    while let Some(line) = tailer.next().await? {
        let line_str = String::from_utf8_lossy(&line);
        print!("{}", line_str);
    }
    Ok(())
}
