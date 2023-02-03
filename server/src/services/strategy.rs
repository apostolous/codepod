use super::executor::Executor;
use std::process::Output;

struct StrategyCommands {
    /// defaults to POSIX `command -v <status>`
    exists: Option<String>,
    status: String,
    create: String,
    graceful_delete: String,
    forceful_delete: String,
}

impl StrategyCommands {
    fn check_binary_exists(&self) -> Result<Output, std::io::Error> {
        let exists = self
            .exists
            .clone()
            .unwrap_or(format!("command -v {}", self.status));
        Executor::exec(&exists)
    }

    fn create_cmd(&self) -> Result<Output, std::io::Error> {
        Executor::exec(&self.create)
    }

    fn graceful_delete_cmd(&self) -> Result<Output, std::io::Error> {
        Executor::exec(&self.graceful_delete)
    }

    fn forceful_delete_cmd(&self) -> Result<Output, std::io::Error> {
        Executor::exec(&self.forceful_delete)
    }

    fn status_cmd(&self) -> Result<Output, std::io::Error> {
        Executor::exec(&self.status)
    }
}

struct Strategy {
    meta: String,
    commands: StrategyCommands,
}
