use std::process::Output;

use super::{executor::Executor, strategy::{StrategyCommands, StrategyCommandsTemplate}};

enum Status {
    None,
    Created,
    Up,
    Down
}

pub struct Pod {
    id: String,
    name: String,
    status: Status,
    commands: StrategyCommands,
}

impl Pod {
    pub fn new(id: &str, name: &str, commands: &StrategyCommandsTemplate) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            status: Status::None,
            commands: commands.new_template(&id)
        }
    }

    fn graceful_delete(&self) -> Result<Output, std::io::Error> {
        Executor::exec(&self.commands.graceful_delete)
    }

    fn forceful_delete(&self) -> Result<Output, std::io::Error> {
        Executor::exec(&self.commands.forceful_delete)
    }

    fn status(&self) -> Result<Output, std::io::Error> {
        Executor::exec(&self.commands.status)
    }
}
