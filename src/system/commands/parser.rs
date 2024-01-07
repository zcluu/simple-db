use crate::database;
use crate::system::commands::r#type::{CommandType, SysCommand};
use crate::system::commands::{normal as NC, system as SC};

pub fn process_sys_command(query: String, db: &mut database::db::Database) {
    let command_type = match SysCommand::new(query.clone()) {
        Ok(v) => v,
        Err(err) => {
            print!("{}", err);
            return;
        }
    };
    match command_type {
        SysCommand::CreateDatabase => match SC::create_db(query.as_str()) {
            Ok(_) => {}
            Err(err) => {
                println!("{}", err)
            }
        },
        SysCommand::UseDatabase => match SC::use_db(query, db) {
            Ok(_) => {}
            Err(err) => {
                println!("{}", err)
            }
        },
        SysCommand::DropDatabase => match SC::drop_db(query) {
            Ok(_) => {}
            Err(err) => {
                println!("{}", err)
            }
        },
        SysCommand::ShowDatabases => SC::show_databases().unwrap(),
        SysCommand::ChangePassword => {}
        SysCommand::HelpTips => SC::help(query),
        SysCommand::SysInfo => {}
    }
}

pub fn process_command(query: String, db: &mut database::db::Database) {
    let command_type = match CommandType::new(query.clone()) {
        Ok(v) => v,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    match command_type {
        CommandType::CreateTable => NC::create_tb(query, db),
        CommandType::Insert => NC::insert_data(query, db),
        CommandType::Select => NC::select_data(query, db),
        CommandType::Delete => NC::delete_data(query, db),
        CommandType::Drop => NC::drop_tb(query, db),
        CommandType::Update => NC::update_data(query, db),
        CommandType::ShowTable => NC::show_tb_data(query, db),
        CommandType::ShowDB => NC::show_all_tbs(db),
        CommandType::TableInfo => NC::show_tb_info(query, db),
        CommandType::System => {
            process_sys_command(query, db);
        }
    }
}
