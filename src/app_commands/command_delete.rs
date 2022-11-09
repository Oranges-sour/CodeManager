use colored::{Color, Colorize};

use crate::command_helper::run_command;
use crate::command_tree::{Command, CommandFn};
use crate::config;

pub struct CommandRemove {
    command: Command,
}

impl CommandRemove {
    pub fn new() -> CommandRemove {
        CommandRemove {
            command: Command::new("remove"),
        }
    }
}

impl CommandFn for CommandRemove {
    fn run_command(&mut self, command_list: &config::ConfigList) -> Result<(), String> {
        let Some(str) = command_list.get_arg(command_list.get_deep() + 1) else {
            return Err("缺失文件名称".to_string());
        };

        if let Err(e) = run_delete(str) {
            return Err(e);
        }
        return Ok(());   
    }

    fn check(&self, key: &String) -> bool {
        self.command.check(key)
    }

    fn add_child(&mut self, command: Box<dyn CommandFn>) {
        self.command.add_child(command);
    }
}

fn run_delete(name: &str) -> Result<(), String> {
    println!("{}>", "删除源代码".color(Color::Cyan));

    let files = match all_files() {
        Err(e) => return Err(e),
        Ok(t) => t,
    };
    let mut has_file = false;
    for it in files.iter() {
        if it == name {
            has_file = true;
        }
    }
    if !has_file {
        return Err("无指定文件".to_string());
    }

    delete_files(name);

    println!("{}", "成功".color(Color::Green));

    Ok(())
}

fn all_files() -> Result<Vec<String>, String> {
    let result = match run_command("ls", &vec![]) {
        Ok(t) => t,
        Err(e) => return Err(e),
    };

    let result = result
        .split("\n")
        .filter(|t| t.len() != 0)
        .map(|t| t.to_string())
        .collect();

    Ok(result)
}

fn delete_files(name: &str) {
    match run_command("rm", &vec!["-rf", name]) {
        Ok(_) => (),
        Err(e) => panic!("rm Error: {}", e),
    }
}
