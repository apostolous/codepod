use std::{fs::read_to_string, process::Output};

use serde::Deserialize;

use super::{executor::Executor, pod::Pod};

#[derive(Deserialize, Clone, Debug)]
pub struct StrategyCommandsTemplate {
    /// defaults to POSIX `command -v <status>`
    pub exists: Option<String>,
    pub status: String,
    pub create: String,
    pub graceful_delete: String,
    pub forceful_delete: String,
}

#[derive(Clone)]
pub struct StrategyCommands {
    pub status: String,
    pub create: String,
    pub graceful_delete: String,
    pub forceful_delete: String,
}

impl From<StrategyCommandsTemplate> for StrategyCommands {
    fn from(value: StrategyCommandsTemplate) -> Self {
        Self {
            status: value.status,
            create: value.create,
            graceful_delete: value.graceful_delete,
            forceful_delete: value.forceful_delete,
        }
    }
}

impl StrategyCommandsTemplate {
    /// Takes this and returns a template-filled copy.
    pub fn new_template(&self, id: &str) -> StrategyCommands {
        let mut me = self.clone();

        me.status = self.status.replace("{ID}", id);
        me.create = self.create.replace("{ID}", id);
        me.graceful_delete = self.graceful_delete.replace("{ID}", id);
        me.forceful_delete = self.forceful_delete.replace("{ID}", id);

        me.into()
    }
}

#[derive(Deserialize, Debug)]
struct Meta {
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct Strategy {
    meta: Meta,
    commands_template: StrategyCommandsTemplate,
}

/// The difference between a Strategy and a Pod
/// A Strategy is merely a parsed version of a `strategy`, while a `Pod` is an instantiation of a `strategy`.
impl Strategy {
    pub fn check_binary_exists(&self) -> Result<Output, std::io::Error> {
        let exists = self
            .commands_template
            .exists
            .clone()
            .unwrap_or(format!("command -v {}", self.commands_template.status));
        Executor::exec(&exists)
    }

    pub fn new_from_file(path: &str) -> Result<Self, std::io::Error> {
        read_to_string(path).and_then(|s| {
            toml::from_str::<Strategy>(&s)
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid Data"))
        })
    }

    pub fn pod_builder(&self, name: &str) -> Result<Pod, std::io::Error> {
        let out = Executor::exec(&self.commands_template.create.replace("{ID}", name))?;
        let s = std::str::from_utf8(&out.stdout).unwrap();

        Ok(Pod::new(s, name, &self.commands_template))
    }
}
