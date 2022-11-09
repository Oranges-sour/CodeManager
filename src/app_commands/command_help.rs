use colored::{Color, Colorize};

use crate::command_tree::{Command, CommandFn};
use crate::config;

pub struct CommandHelp {
    command: Command,
}

impl CommandHelp {
    pub fn new() -> CommandHelp {
        CommandHelp {
            command: Command::new("help"),
        }
    }
}

impl CommandFn for CommandHelp {
    fn run_command(&mut self, _: &config::ConfigList) -> Result<(), String> {
        print_help();

        Ok(())
    }

    fn check(&self, key: &String) -> bool {
        self.command.check(key)
    }

    fn add_child(&mut self, command: Box<dyn CommandFn>) {
        self.command.add_child(command);
    }
}

fn print_help() {
    println!("{}>{}", "帮助".color(Color::Cyan), "将列出所有可用指令");

    let cmds = [
        ("-指令-", "-参数-", "-注释-"),
        ("new", "name [-ovride]", "创建新的源代码; [-ovride]可选的覆盖已有文件"),
        ("remove", "name", "删除源代码"),
        ("build", "name [-O2] [-inside]", "编译源代码; [-O2]可选的O2优化编译, [-inside]可选的内建编译(供VSCode使用, 手动调用时无需添加)"),
        ("collect", "[-zip]->[name]", "收集所有cpp文件到文件夹(默认以时间命名); [-zip]可选的打包成zip, [name]可选的自定义zip文件名称"),
        ("split", "name", "将以name命名的zip文件打散到指令能够执行的文件, 注意, 重名的文件将被替换"),
    ];

    for &it in cmds.iter() {
        println!(
            "{:8} {: <24}{:5}",
            it.0.color(Color::Green),
            it.1.color(Color::BrightBlue),
            it.2
        );
    }
}
