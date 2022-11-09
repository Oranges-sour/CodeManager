use colored::{Color, Colorize};

use crate::app_commands::CommandNew;
use crate::command_helper::run_command;
use crate::command_tree::{Command, CommandFn};
use crate::config;

use rand::Rng;

use regex::Regex;

pub struct CommandSplit {
    command: Command,
}

impl CommandSplit {
    pub fn new() -> CommandSplit {
        CommandSplit {
            command: Command::new("split"),
        }
    }
}

impl CommandFn for CommandSplit {
    fn run_command(&mut self, command_list: &config::ConfigList) -> Result<(), String> {
        let Some(str) = command_list.get_arg(command_list.get_deep() + 1) else {
            return Err("缺失的文件名称".to_string());
        };

        if let Err(e) = run_split(str) {
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

fn run_split(name: &str) -> Result<(), String> {
    println!("{}>", "分散源代码".color(Color::Cyan));

    if let Err(_) = run_command("find", &vec![name]) {
        return Err("没有找到对应的zip文件, 是否缺失了文件后缀名?".to_string());
    }

    let temp_folder = generate_random_str(5);
    run_command("mkdir", &vec![temp_folder.as_str()]).unwrap();

    run_command("unzip", &vec!["-d", temp_folder.as_str(), name]).unwrap();

    let Ok(t) = run_command("find", &vec![format!("{}/", temp_folder).as_str(), "-name", "*.cpp"]) else {
        return Err("内部错误, 寻找文件失败".to_string());
    };

    //文件名, 有完整路径
    let files_full: Vec<_> = t.split("\n").collect();

    let files = match get_file_name(&files_full) {
        Ok(t) => t,
        Err(e) => return Err(e),
    };

    for it in files.iter() {
        if let Err(e) = run_new_inside(&vec![&it.0, "-ovride"]) {
            return Err(e);
        }

        run_command("cp", &vec![it.1, &it.0]).unwrap();
    }

    run_command("rm", &vec!["-rf", temp_folder.as_str()]).unwrap();

    println!("{}", "成功".color(Color::Green));

    Ok(())
}

fn run_new_inside(args: &Vec<&str>) -> Result<(), String> {
    let mut cmd_new = CommandNew::new();
    let conf = config::Config::from_user(&args.iter().map(|t| t.to_string()).collect());
    let conf_list = config::ConfigList::new(&conf, 0);

    cmd_new.run_command(&conf_list)
}

fn generate_random_str(len: usize) -> String {
    const CHARS: [char; 36] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
        'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    let gen_num = || -> usize { rand::thread_rng().gen_range(0..26) };

    let mut str = String::new();

    let mut len = len;

    while len != 0 {
        str.push(CHARS[gen_num()]);
        len -= 1;
    }

    str
}

fn get_file_name<'a>(files: &Vec<&'a str>) -> Result<Vec<(String, &'a str)>, String> {
    let mut result = Vec::new();

    let r = Regex::new(r"/{1}\w+\.cpp").unwrap();

    for name in files.iter() {
        if name.len() == 0 {
            continue;
        }
        if !r.is_match(name) {
            return Err(format!("文件名称正则匹配失败:{}", name).to_string());
        }

        let mut cap = r.captures_iter(name);
        let str = format!("{}", &cap.next().unwrap()[0]);
        let new_name = &str[1..=str.len() - 5];
        result.push((new_name.to_string(), *name));
    }

    Ok(result)
}
