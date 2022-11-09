use code::app_commands;
use code::App;
use colored::{Color, Colorize};

fn main() {
    let mut app = App::new();

    init_commands(&mut app);

    if let Err(str) = app.run() {
        println!("{}", str.color(Color::Red));
    }
}

fn init_commands(app: &mut App) {
    let cmd = Box::new(app_commands::CommandHelp::new());
    app.add_child(cmd);

    let cmd = Box::new(app_commands::CommandNew::new());
    app.add_child(cmd);

    let cmd = Box::new(app_commands::CommandRemove::new());
    app.add_child(cmd);

    let cmd = Box::new(app_commands::CommandCompile::new());
    app.add_child(cmd);

    let cmd = Box::new(app_commands::CommandCollect::new());
    app.add_child(cmd);

    let cmd = Box::new(app_commands::CommandSplit::new());
    app.add_child(cmd);
}
