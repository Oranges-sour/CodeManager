use crate::config;

pub struct CommandTree {
    child: MainCommand,
}

impl CommandTree {
    pub fn new() -> CommandTree {
        CommandTree {
            child: MainCommand::new(),
        }
    }

    pub fn add_child(&mut self, command: Box<dyn CommandFn>) {
        self.child.add_child(command);
    }

    pub fn run_command(&mut self, config: &config::ConfigList) -> Result<(), String> {
        self.child.run_command(config)
    }
}

struct MainCommand {
    command: Command,
}

impl MainCommand {
    fn new() -> MainCommand {
        MainCommand {
            command: Command::new(""),
        }
    }
}

impl CommandFn for MainCommand {
    fn run_command(&mut self, command_list: &config::ConfigList) -> Result<(), String> {
        //...
        self.command.run_command(command_list)
    }

    fn check(&self, _: &String) -> bool {
        true
    }

    fn add_child(&mut self, command: Box<dyn CommandFn>) {
        self.command.add_child(command);
    }
}

pub trait CommandFn {
    fn run_command(&mut self, command_list: &config::ConfigList) -> Result<(), String>;
    fn check(&self, key: &String) -> bool;
    fn add_child(&mut self, command: Box<dyn CommandFn>);
}

pub struct Command {
    children: Vec<Box<dyn CommandFn>>,
    key: String,
}

impl Command {
    pub fn new(key: &str) -> Command {
        Command {
            children: Vec::new(),
            key: String::from(key),
        }
    }

    pub fn check(&self, key: &String) -> bool {
        self.key == *key
    }

    pub fn add_child(&mut self, child: Box<dyn CommandFn>) {
        self.children.push(child);
    }

    fn child_iter_mut(&mut self) -> std::slice::IterMut<Box<dyn CommandFn>> {
        self.children.iter_mut()
    }

    pub fn run_command(&mut self, command_list: &config::ConfigList) -> Result<(), String> {
        let mut new_list = command_list.clone();
        new_list.set_deep(command_list.get_deep() + 1);
        let new_list = new_list;

        let mut match_any = false;

        for it in self.child_iter_mut() {
            let Some(key) = command_list.get_arg(command_list.get_deep() + 1) else {
                return Err("<run_command>:没有找到对应深度的参数".to_string());
            };

            if it.check(key) {
                match_any = true;

                if let Err(str) = it.run_command(&new_list) {
                    return Err(str);
                }
            }
        }

        if !match_any {
            return Err("没有匹配到指令".to_string());
        }

        Ok(())
    }
}
