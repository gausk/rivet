mod command;
mod prompt;

use crate::command::Command;
use crate::prompt::shell_prompt;
use anyhow::Result;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter, stdin, stdout};

#[tokio::main]
async fn main() {
    loop {
        if let Err(e) = run_shell().await {
            eprintln!("{}", e);
        }
    }
}

async fn run_shell() -> Result<()> {
    let stdin = stdin();
    let stdout = stdout();

    let mut reader = BufReader::new(stdin).lines();
    let mut writer = BufWriter::new(stdout);

    let mut current_dir = std::env::current_dir()?;

    writer
        .write_all(b"Welcome to Rivet: Secure Shell in rust!\n")
        .await?;
    writer
        .write_all(shell_prompt(&current_dir).as_bytes())
        .await?;
    writer.flush().await?;
    while let Ok(Some(line)) = reader.next_line().await {
        match Command::try_from(line.trim()) {
            Ok(command) => {
                let _ = command.execute(&mut current_dir).await.inspect_err(|e| eprintln!("{}", e));
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
        writer
            .write_all(shell_prompt(&current_dir).as_bytes())
            .await?;
        writer.flush().await?;
    }
    Ok(())
}
