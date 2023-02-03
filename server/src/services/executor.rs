use std::{process::{Command, Output}, io};

pub struct Executor {}

impl Executor {
  /// Spawns a new shell and executes a command.
  pub fn exec(cmd: &str) -> io::Result<Output> {
    let cmd = cmd.split_ascii_whitespace().collect::<Vec<&str>>();
    Command::new(cmd[0])
      .args(&cmd[1..])
      .output()
  }
}