use std::sync::Arc;
use pumpkin::command::tree::CommandTree;
use pumpkin::plugin::Context;

use crate::command::{Login, Register, Changepassword};
use crate::event::MyJoinHandler;

const NAMES: &str = "CatSeedLogin";
const DESCRIPTION: &str = "Login plugin for CatSeed";

pub async fn initialize_plugin(server: Arc<Context>) -> Result<(), String> {
    pumpkin::init_log!();
    log::info!("CatSeedLogin-rust has been loaded!");
    server.register_event(Arc::new(MyJoinHandler), pumpkin::plugin::EventPriority::Lowest, true).await;
    
    // 创建登录命令树
    let login_tree = CommandTree::new(["login"], "Login command");
    
    // 创建注册命令树
    let register_tree = CommandTree::new(["register"], "Register command");
    
    // 创建修改密码命令树
    let changepassword_tree = CommandTree::new(["changepassword"], "Change password command");
    
    // 注册命令树
    server.register_command(login_tree, "login").await;
    server.register_command(register_tree, "register").await;
    server.register_command(changepassword_tree, "changepassword").await;
    
    Ok(())
}
