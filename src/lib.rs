pub mod app_commands;


mod command_helper;
mod command_tree;

mod config;

pub struct App {
    command_tree: command_tree::CommandTree,
    config: config::Config,
}

impl App {
    pub fn new() -> App {
        App {
            command_tree: command_tree::CommandTree::new(),
            config: config::Config::new(),
        }
    }

    pub fn add_child(&mut self, command: Box<dyn command_tree::CommandFn>) {
        self.command_tree.add_child(command);
    }

    pub fn run(&mut self) -> Result<(), String> {
        let config_list = config::ConfigList::new(&self.config, 0);
        self.command_tree.run_command(&config_list)
    }
}
