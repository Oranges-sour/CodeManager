use colored::{Color, Colorize};

use crate::command_helper::run_command;
use crate::command_tree::{Command, CommandFn};
use crate::config;

use regex::Regex;

pub struct CommandCompile {
    command: Command,
}

impl CommandCompile {
    pub fn new() -> CommandCompile {
        CommandCompile {
            command: Command::new("build"),
        }
    }
}

struct ExArgs {
    compile_o2: bool,
    build_inside: bool,
}

impl ExArgs {
    fn from(compile_o2: bool, build_inside: bool) -> ExArgs {
        ExArgs {
            compile_o2,
            build_inside,
        }
    }
}

impl CommandFn for CommandCompile {
    fn run_command(&mut self, command_list: &config::ConfigList) -> Result<(), String> {
        let deep = command_list.get_deep();

        let Some(file) = command_list.get_arg(deep + 1) else {
            return Err("缺失文件名称".to_string());
        };

        let args = match compare_ex_args(command_list) {
            Ok(t) => t,
            Err(e) => return Err(e),
        };

        println!("{}>", "编译源代码".color(Color::Cyan));
        if args.build_inside {
            run_compile_inside(file, args.compile_o2)
        } else {
            run_compile(file, args.compile_o2)
        }
    }

    fn check(&self, key: &String) -> bool {
        self.command.check(key)
    }

    fn add_child(&mut self, command: Box<dyn CommandFn>) {
        self.command.add_child(command);
    }
}

fn compare_ex_args(command_list: &config::ConfigList) -> Result<ExArgs, String> {
    let mut ex_args = Vec::new();

    let deep = command_list.get_deep();
    ex_args.push(command_list.get_arg(deep + 2));
    ex_args.push(command_list.get_arg(deep + 3));

    let (mut compile_o2, mut build_inside) = (false, false);

    let mut arg_list = [("-O2", &mut compile_o2), ("-inside", &mut build_inside)];

    for it in ex_args.iter() {
        let Some(t) = it else {
            continue;
        };

        let mut suc = false;
        for comp in arg_list.iter_mut() {
            if *t == comp.0 {
                suc = true;
                *comp.1 = true;
            }
        }

        if !suc {
            return Err(format!("未知的附加参数:{}", t));
        }
    }

    Ok(ExArgs::from(compile_o2, build_inside))
}

fn run_compile(name: &str, compile_o2: bool) -> Result<(), String> {
    if let Err(e) = has_file(name) {
        return Err(e);
    }

    if compile_o2 {
        println!("{}", "使用O2优化编译".color(Color::BrightBlue));
    }

    if let Err(e) = compile_file(name, compile_o2) {
        return Err(e);
    }

    println!("{}", "成功".color(Color::Green));

    Ok(())
}

fn run_compile_inside(name: &str, compile_o2: bool) -> Result<(), String> {
    println!("{}", "使用内建编译".color(Color::BrightBlue));

    let r = Regex::new(r"/{1}\w+\.cpp").unwrap();
    if !r.is_match(name) {
        return Err("文件名称正则匹配失败".to_string());
    }

    let mut cap = r.captures_iter(name);
    let str = format!("{}", &cap.next().unwrap()[0]);
    let name = &str[1..=str.len() - 5];

    run_compile(name, compile_o2)
}

fn has_file(name: &str) -> Result<(), String> {
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

fn compile_file(name: &str, o2: bool) -> Result<(), String> {
    let src = format!("{}/{}.cpp", name, name);
    let opt = format!("{}/main", name);

    let arg_o2 = vec![
        "-O2",        //
        "-std=c++14", //
        "-g",         //
        src.as_str(), //
        "-o",         //
        opt.as_str(), //
    ];

    let arg_non_o2 = vec![
        "-std=c++14", //
        "-g",         //
        src.as_str(), //
        "-o",         //
        opt.as_str(), //
    ];

    //先删除原先的文件
    if let Err(e) = run_command("rm", &vec!["-rf", opt.as_str()]) {
        panic!("{}:{}", "删除文件错误", e);
    }

    if o2 {
        if let Err(e) = run_command("g++", &arg_o2) {
            return Err(e + "\n编译失败");
        }
    } else {
        if let Err(e) = run_command("g++", &arg_non_o2) {
            return Err(e + "\n编译失败");
        }
    }

    Ok(())
}
