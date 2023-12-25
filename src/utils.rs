use std::fs::File;
use std::io;
use std::io::Write;
use sha2::{Sha256, Digest};
use hex;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Password {
    hashed_password: String,
}

impl Password {
    pub fn new(password: &str) -> Password {
        let hashed_password = Password::hash_password(password);
        Password { hashed_password }
    }

    pub fn set_password(&mut self, new_password: &str) {
        self.hashed_password = Password::hash_password(new_password);
    }

    pub fn check_password(&self, password: &str) -> bool {
        Password::hash_password(password) == self.hashed_password
    }

    fn hash_password(password: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        hex::encode(hasher.finalize())
    }
}

pub enum CommandType {
    CreateTable,
    Insert,
    Select,
    ShowTable,
    System,
}

impl CommandType {
    pub fn new(command: String) -> CommandType {
        let vars = command.split(" ").collect::<Vec<&str>>();
        match vars[0].to_lowercase().as_str() {
            "create" => CommandType::CreateTable,
            "insert" => CommandType::Insert,
            "select" => CommandType::Select,
            "show" => CommandType::ShowTable,
            "sys" => CommandType::System,
            _ => panic!("Invalid command.")
        }
    }
}

pub enum SysCommand {
    CreateDatabase,
    UseDatabase,
    DropDatabase,
    ShowDatabases,
    ChangePassword,
    SysInfo,
}

impl SysCommand {
    pub fn new(command: String) -> SysCommand {
        let vars = command.split(" ").collect::<Vec<&str>>();
        match vars[1].to_lowercase().as_str() {
            "createdb" => SysCommand::CreateDatabase,
            "usedb" => SysCommand::UseDatabase,
            "dropdb" => SysCommand::DropDatabase,
            "showdb" => SysCommand::ShowDatabases,
            "changepwd" => SysCommand::ChangePassword,
            "showsys" => SysCommand::SysInfo,
            _ => panic!("Invalid command.")
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DbSystem {
    sys_name: String,
    sys_username: String,
    sys_password: Password,
}

impl DbSystem {
    pub fn load_cfg(&mut self) -> io::Result<()> {
        let file = File::open("cfg.bin")?;
        *self = serde_json::from_reader(file)?;
        Ok(())
    }
    pub fn new() -> DbSystem {
        let mut sys = DbSystem {
            sys_name: "".to_string(),
            sys_username: "".to_string(),
            sys_password: Password { hashed_password: "".to_string() },
        };
        sys.load_cfg().unwrap();
        sys
    }

    pub fn login(&self, username: String, password: String) -> bool {
        let mut status = true;
        status &= username == self.sys_username;
        status &= self.sys_password.check_password(&*password.to_string());
        status
    }

    pub fn update_info(&self) -> io::Result<()> {
        let serialized_data = serde_json::to_string(&self)?;
        let mut file = File::create("cfg.bin")?;
        file.write_all(serialized_data.as_bytes())?;
        Ok(())
    }
}
