use crate::database;
use crate::system::commands::r#type;
use crate::system::commands::{normal as NC, system as SC};

pub fn process_sys_command(query: String, db: &mut database::db::Database) {
    match r#type::SysCommand::new(query.clone()) {
        r#type::SysCommand::CreateDatabase => SC::create_db(query).unwrap(),
        r#type::SysCommand::UseDatabase => SC::use_db(query, db).unwrap(),
        r#type::SysCommand::DropDatabase => SC::drop_db(query).unwrap(),
        r#type::SysCommand::ShowDatabases => SC::show_databases().unwrap(),
        r#type::SysCommand::ChangePassword => {}
        r#type::SysCommand::SysInfo => {}
    }
}

pub fn process_command(query: String, db: &mut database::db::Database) {
    match r#type::CommandType::new(query.clone()) {
        r#type::CommandType::CreateTable => NC::create_tb(query, db),
        r#type::CommandType::Insert => NC::insert_data(query, db),
        r#type::CommandType::Select => NC::select_data(query, db),
        r#type::CommandType::Delete => NC::delete_data(query, db),
        r#type::CommandType::Drop => NC::drop_tb(query, db),
        r#type::CommandType::Update => NC::update_data(query, db),
        r#type::CommandType::ShowTable => NC::show_tb_data(query, db),
        r#type::CommandType::ShowDB => NC::show_all_tbs(db),
        r#type::CommandType::TableInfo => NC::show_tb_info(query, db),
        r#type::CommandType::System => {
            process_sys_command(query, db);
        }
    }
}
