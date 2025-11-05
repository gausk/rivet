mod prompt;

use crate::prompt::shell_prompt;
use anyhow::Result;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter, stdin, stdout};

#[tokio::main]
async fn main() -> Result<()> {
    let stdin = stdin();
    let stdout = stdout();

    let mut reader = BufReader::new(stdin).lines();
    let mut writer = BufWriter::new(stdout);

    let current_dir = std::env::current_dir()?;

    writer
        .write_all(b"Welcome to Rivet: Secure Shell in rust!\n")
        .await?;
    writer
        .write_all(shell_prompt(&current_dir).as_bytes())
        .await?;
    writer.flush().await?;
    while let Ok(Some(line)) = reader.next_line().await {
        writer.write_all(line.as_bytes()).await?;
        writer.write_all(b"\n").await?;
        writer
            .write_all(shell_prompt(&current_dir).as_bytes())
            .await?;
        writer.flush().await?;
    }
    Ok(())
}
