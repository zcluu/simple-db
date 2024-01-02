pub enum CommandType {
    CreateTable,
    Insert,
    Select,
    Delete,
    Drop,
    Update,
    ShowTable,
    ShowDB,
    TableInfo,
    System,
}

impl CommandType {
    pub fn new(command: String) -> CommandType {
        let vars = command.split(" ").collect::<Vec<&str>>();
        match vars[0].to_lowercase().as_str() {
            "create" => CommandType::CreateTable,
            "insert" => CommandType::Insert,
            "select" => CommandType::Select,
            "delete" => CommandType::Delete,
            "drop" => CommandType::Drop,
            "update" => CommandType::Update,
            "showtb" => CommandType::ShowTable,
            "showdb" => CommandType::ShowDB,
            "tableinfo" => CommandType::TableInfo,
            "sys" => CommandType::System,
            _ => panic!("Invalid command."),
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
            _ => panic!("Invalid command."),
        }
    }
}
