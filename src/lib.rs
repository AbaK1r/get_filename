use clipboard_win::{Clipboard, formats, Setter};
use std::ops::Add;
use winreg::RegKey;
use winreg::enums::*;

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
                message: String::from("get file name")
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
                Err(_) => return Err("Can't write clipboard.")
            };
        },
        Err(_) => return Err("Can't open clipboard.")
    };
    Ok(())
}

pub fn add_registry(config: &Config) -> Result<(), &'static str> {
    let masssege = config.message.clone();
    match std::env::current_exe() {
        Ok(this) => {
            match this.into_os_string().into_string() {
                Ok(this) => {
                    let this = this.replace("\\\\", "\\").add(" \"%1\"");
                    match _add_registry("*\\shell\\get file name", &this, &masssege) {
                        Ok(_) => { println!("Registry for file is done.") },
                        Err(_) => return Err("Can't write to the registry for file, maybe you need to get admin privileges.")
                    }
                    match _add_registry("Directory\\shell\\get file name", &this, &masssege) {
                        Ok(_) => { println!("Registry for folder is done.") },
                        Err(_) => return Err("Can't write to the registry for folder, maybe you need to get admin privileges.")
                    }
                },
                Err(_) => return Err("Can't find courrent path.")
            }
        },
        Err(_) => return Err("Can't find courrent path.")
    }
    println!("Success!");
    Ok(())
}

fn _add_registry(registry_dir: &str, this: &String, message: &String) -> Result<(), ()> {
    let my_key = RegKey::predef(HKEY_CLASSES_ROOT);
    match my_key.create_subkey(registry_dir) {
        Ok((my_key, _)) => {
            my_key.set_value("", message).unwrap();
            match my_key.create_subkey("command") {
                Ok((my_key, _)) => {
                    my_key.set_value("", this).unwrap();
                },
                Err(_) => return Err(())
            }
        },
        Err(_) => return Err(())
    }
    Ok(())
}

pub fn remove_registry() -> Result<(), &'static str> {
    let my_key = RegKey::predef(HKEY_CLASSES_ROOT);
    match my_key.delete_subkey_all("*\\shell\\get file name") {
        Ok(_) => { println!("Success to delete the registry for file.") },
        Err(_) => return Err("Can't delete the registry for file, maybe you need to get admin privileges or the registry is not exist.")
    }
    match my_key.delete_subkey_all("Directory\\shell\\get file name") {
        Ok(_) => { println!("Success to delete the registry for folder.") },
        Err(_) => return Err("Can't delete the registry for folder, maybe you need to get admin privileges or the registry is not exist.")
    }
    println!("Success!");
    Ok(())
}
