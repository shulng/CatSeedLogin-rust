use std::sync::Arc;
use pumpkin::command::tree::CommandTree;
use pumpkin::plugin::Context;
use pumpkin::command::tree::builder::literal;
use pumpkin_util::PermissionLvl;
use crate::command::{Login, Register, Changepassword, Bindemail, Resetpassword};
use crate::event::MyJoinHandler;

const NAMES: &str = "CatSeedLogin";
const DESCRIPTION: &str = "Login plugin for CatSeed";

pub async fn initialize_plugin(server: Arc<Context>) -> Result<(), String> {
    pumpkin::init_log!();
    log::info!("CatSeedLogin-rust has been loaded!");
    server.register_event(Arc::new(MyJoinHandler), pumpkin::plugin::EventPriority::Lowest, true).await;
    
    // 创建登录命令树
    let login_tree = CommandTree::new(["login"], "Login command")
       .then(literal("login").execute(Login));
       
    // 创建注册命令树
    let register_tree = CommandTree::new(["register"], "Register command")
       .then(literal("register").execute(Register));
       
    // 创建修改密码命令树
    let changepassword_tree = CommandTree::new(["changepassword"], "Change password command")
       .then(literal("changepassword").execute(Changepassword));
       
    // 创建绑定邮箱命令树
    let bindemail_tree = CommandTree::new(["bindemail"], "bindemail command")
       .then(literal("bindemail").execute(Bindemail));
       
    // 创建忘记密码重置命令树
    let resetpassword_tree = CommandTree::new(["resetpassword"], "resetpassword command")
       .then(literal("resetpassword").execute(Resetpassword));
    Ok(())
}
