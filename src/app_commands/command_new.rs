use colored::{Color, Colorize};

use crate::command_helper::run_command;
use crate::command_tree::{Command, CommandFn};
use crate::config;

use std::fs::File;
use std::io::Write;

pub struct CommandNew {
    command: Command,
}

impl CommandNew {
    pub fn new() -> CommandNew {
        CommandNew {
            command: Command::new("new"),
        }
    }
}

impl CommandFn for CommandNew {
    fn run_command(&mut self, command_list: &config::ConfigList) -> Result<(), String> {
        let Some(str) = command_list.get_arg(command_list.get_deep() + 1) else {
            return Err("缺失的文件名称".to_string());
        };

        //强制覆盖
        let mut ovride = false;

        if let Some(str) = command_list.get_arg(command_list.get_deep() + 2) {
            if str == "-ovride" {
                ovride = true;
            } else {
                return Err(format!("未知的附加参数:{}", str));
            }
        }

        if let Err(e) = run_new(str, ovride) {
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

fn run_new(name: &str, ovride: bool) -> Result<(), String> {
    println!("{}>{}", "新建源代码".color(Color::Cyan), name);

    let files = match all_files() {
        Err(e) => return Err(e),
        Ok(t) => t,
    };
    for it in files.iter() {
        if it == name && !ovride {
            return Err("文件名重复".to_string());
        }
    }

    run_command("rm", &vec!["-rf", name]).unwrap();

    new_files(name);

    println!("{}", "成功".color(Color::Green));

    Ok(())
}

fn new_files(name: &str) {
    match run_command("mkdir", &vec![name]) {
        Ok(_) => (),
        Err(e) => panic!("new_files Error: {}", e),
    }

    let mut file = File::create(format!("{}/{}.cpp", name, name)).unwrap();
    file.write(b"int main() { return 0; }").unwrap();
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
