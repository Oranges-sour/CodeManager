use std::env;

pub struct Config {
    args: Vec<String>,
}

impl Config {
    pub fn new() -> Config {
        let arg: Vec<String> = env::args().collect();

        let mut cf = Config {
            args: arg[1..].to_vec(),
        };
        cf.args.insert(0, "^".to_string());

        cf
    }

    pub fn from_user(args: &Vec<String>) -> Config {
        let mut cf = Config { args: args.clone() };
        cf.args.insert(0, "^".to_string());
        cf
    }

    pub fn get_args(&self) -> &Vec<String> {
        &self.args
    }
}

#[derive(Clone)]
pub struct ConfigList<'a> {
    args: &'a Vec<String>,
    now_deep: usize,
}

impl<'a> ConfigList<'a> {
    pub fn new(config: &Config, now_deep: usize) -> ConfigList {
        ConfigList {
            args: config.get_args(),
            now_deep: now_deep,
        }
    }

    pub fn get_arg(&self, deep: usize) -> Option<&String> {
        if let Some(str) = self.args.get(deep) {
            return Some(str);
        }
        None
    }

    pub fn get_deep(&self) -> usize {
        self.now_deep
    }

    pub fn set_deep(&mut self, deep: usize) {
        self.now_deep = deep;
    }
}
