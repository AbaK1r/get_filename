use get_filename::{AppMode, Config, main_app, add_registry, remove_registry};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut config = Config::parse_config(args).unwrap_or_else(|err| {
        println!("Error: {err}");
        process::exit(1);
    });
    match config.app_mode {
        AppMode::MainApp => main_app(&mut config).unwrap_or_else(|err| {
            println!("Error: {err}");
            process::exit(2);
        }),
        AppMode::AddRegistry => add_registry(&config).unwrap_or_else(|err| {
            println!("Error: {err}");
            process::exit(3);
        }),
        AppMode::RemoveRegistry => remove_registry().unwrap_or_else(|err| {
            println!("Error: {err}");
            process::exit(4);
        }),
    };
}
