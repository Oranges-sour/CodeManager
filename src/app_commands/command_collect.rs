use chrono::prelude::*;
use colored::{Color, Colorize};

use crate::command_helper::run_command;
use crate::command_tree::{Command, CommandFn};
use crate::config;

pub struct CommandCollect {
    command: Command,
}

impl CommandCollect {
    pub fn new() -> CommandCollect {
        CommandCollect {
            command: Command::new("collect"),
        }
    }
}

impl CommandFn for CommandCollect {
    fn run_command(&mut self, command_list: &config::ConfigList) -> Result<(), String> {
        let folder = match run_collect() {
            Ok(t) => t,
            Err(e) => return Err(e),
        };

        let mut pack_zip = false;

        let deep = command_list.get_deep();
        if let Some(t) = command_list.get_arg(deep + 1) {
            if t == "-zip" {
                pack_zip = true;
            } else {
                return Err(format!("未知的附加参数:{}", t));
            }
        }

        if pack_zip {
            let mut zip_name = &folder;
            let mut user_name = false;
            
            if let Some(t) = command_list.get_arg(deep + 2) {
                zip_name = t;
                user_name = true;
            }

            if user_name {
                run_command("mv", &vec![folder.as_str(), zip_name.as_str()]).unwrap();
            }

            run_command(
                "zip",
                &vec!["-r", "-m", &format!("{}.zip", zip_name), zip_name.as_str()],
            )
            .unwrap();
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

fn run_collect() -> Result<String, String> {
    println!("{}>", "收集源代码".color(Color::Cyan));

    let files = match all_files() {
        Ok(t) => t,
        Err(e) => return Err(e),
    };

    let mut source_file = Vec::new();

    for f in files.iter() {
        let find_name = format!("{}/{}.cpp", f, f);

        if let Ok(_) = run_command("find", &vec![find_name.as_str()]) {
            source_file.push(find_name);
        }
    }

    if source_file.is_empty() {
        return Err("没有需要收集的源代码".to_string());
    }

    let local_time = Local::now();
    let folder_name = local_time.format("backup_%Y%m%d_%H%M%S").to_string();
    let return_result = folder_name.clone();
    run_command("mkdir", &vec![folder_name.as_str()]).unwrap();

    let folder_name = format!("{}/", folder_name);

    let mut args = Vec::new();
    args.push("-r");
    for it in source_file.iter() {
        args.push(it);
    }
    args.push(folder_name.as_str());

    run_command("cp", &args).unwrap();

    println!("{}", "成功".color(Color::Green));

    Ok(return_result)
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
