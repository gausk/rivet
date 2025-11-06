#![allow(unused)]
use anyhow::bail;

#[derive(Debug, Clone)]
pub enum Command {
    Exit,
    Echo(String),
    Ls(Vec<String>),
    Pwd,
    Cd(String),
    Touch(Vec<String>),
    Rm(Vec<String>),
    Cat(Vec<String>),
}

impl TryFrom<&str> for Command {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let values: Vec<&str> = value.split(' ').collect();
        Ok(match values[0] {
            "exit" => Command::Exit,
            "echo" => Command::Echo(values[1..].join(" ")),
            "ls" => Command::Ls(values[1..].iter().map(|x| x.to_string()).collect()),
            "pwd" => {
                if values.len() > 1 {
                    bail!("pwd: too many arguments")
                } else {
                    Command::Pwd
                }
            }
            "cd" => {
                if values.len() != 2 {
                    bail!("cd: too many arguments")
                } else {
                    Command::Cd(values[1].to_string())
                }
            }
            "touch" => Command::Touch(values[1..].iter().map(|x| x.to_string()).collect()),
            "rm" => Command::Rm(values[1..].iter().map(|x| x.to_string()).collect()),
            "cat" => Command::Cat(values[1..].iter().map(|x| x.to_string()).collect()),
            other => bail!("Unknown command: {}", other),
        })
    }
}

impl Command {
    pub async fn execute(&self) -> Result<(), anyhow::Error> {
        match self {
            Command::Exit => std::process::exit(0),
            Command::Echo(args) => println!("{args}"),
            Command::Ls(args) => {
                if args.is_empty() {
                    let entries = std::fs::read_dir(".")?;
                    for entry in entries {
                        let entry = entry?;
                        println!("{entry:?}");
                    }
                } else {
                    for arg in args {
                        let mut entries = tokio::fs::read_dir(arg).await?;
                        while let Some(entry) = entries.next_entry().await? {
                            println!("{entry:?}");
                        }
                    }
                }
            }
            _ => (),
        }
        Ok(())
    }
}
