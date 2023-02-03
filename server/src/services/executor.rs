use std::{process::{Command, Output}, io};

pub struct Executor {}

impl Executor {
  /// Spawns a new shell and executes a command.
  pub fn exec(cmd: &str) -> io::Result<Output> {
    Command::new("sh")
      .args(cmd.split_ascii_whitespace())
      .output()
  }
}