use clipboard_win::{Clipboard, formats, Setter};

pub enum AppMode {
    AddRegistry,
    RemoveRegistry,
    MainApp,
}

pub struct Config {
    pub app_mode: AppMode,
    message: String
}

impl Config {
    pub fn parse_config(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("The number of parameters is incorrect.")
        }
        match &args[1] as &str {
            "build" => Ok(Config {
                app_mode: AppMode::AddRegistry,
                message: if args.len() == 2 { String::from("get file name") } else { args[2].clone() }
            }),
            "unbuild" => Ok(Config {
                app_mode: AppMode::RemoveRegistry,
                message: if args.len() == 2 { String::from("get file name") } else { args[2].clone() }
            }),
            _ => {
                println!("Copied <{}> to the clipboard", args[1]);
                Ok(Config {
                    app_mode: AppMode::MainApp,
                    message: args[1].clone()
                })
            },
        }
    }
}

pub fn main_app(config: &mut Config) -> Result<(), &'static str> {
    let file_path = config.message.replace("\\", "/");
    match Clipboard::new_attempts(10) {
        Ok(_clip) => {
            match formats::Unicode.write_clipboard(&file_path) {
                Ok(_) => {},
                Err(_) => return Err("Error: Can't write clipboard.")
            };
        },
        Err(_) => return Err("Error: Can't open clipboard.")
    };
    Ok(())
}

pub fn add_registry(config: &Config) -> Result<(), &'static str> {
    Ok(())
}

pub fn remove_registry(config: &Config) -> Result<(), &'static str> {
    Ok(())
}
