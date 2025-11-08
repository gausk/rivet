use anyhow::bail;
use std::path::Path;

#[derive(Debug, Clone)]
pub enum Command<'a> {
    Exit,
    Echo(Option<&'a str>),
    Ls(Option<&'a str>),
    Pwd,
}

impl<'a> TryFrom<&'a str> for Command<'a> {
    type Error = anyhow::Error;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (cmd, args) = value
            .split_once(' ')
            .map(|(x, y)| (x, Some(y)))
            .unwrap_or((value, None));
        Ok(match cmd {
            "exit" => Command::Exit,
            "echo" => Command::Echo(args),
            "ls" => Command::Ls(args),
            "pwd" => {
                if args.is_some() {
                    bail!("pwd: too many arguments")
                } else {
                    Command::Pwd
                }
            }
            other => bail!("Unknown command: {}", other),
        })
    }
}

impl Command<'_> {
    pub async fn execute(&self, curr_dir: &Path) -> Result<(), anyhow::Error> {
        match self {
            Command::Exit => std::process::exit(0),
            Command::Echo(args) => println!("{}", args.unwrap_or_default()),
            Command::Ls(args) => {
                if let Some(args) = args {
                    for arg in args.split_whitespace() {
                        let metadata = match tokio::fs::metadata(arg).await {
                            Ok(metadata) => metadata,
                            Err(e) => {
                                eprintln!("ls: {arg}: {e}");
                                continue;
                            }
                        };

                        if metadata.is_dir() {
                            let mut entries = tokio::fs::read_dir(arg).await?;
                            while let Some(entry) = entries.next_entry().await? {
                                print!("{}    ", entry.file_name().to_string_lossy());
                            }
                            println!();
                        } else {
                            println!("{}", arg);
                        }
                    }
                } else {
                    let entries = std::fs::read_dir(".")?;
                    for entry in entries {
                        let entry = entry?;
                        print!("{}    ", entry.file_name().to_string_lossy());
                    }
                    println!();
                }
            }
            Command::Pwd => {
                println!("{}", curr_dir.display());
            }
        }
        Ok(())
    }
}
