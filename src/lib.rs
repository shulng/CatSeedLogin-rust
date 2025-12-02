mod command;
mod event;
mod init;

// 保留必要的导入
use pumpkin_protocol::ConnectionState::Login as ProtocolLogin;
use std::sync::Arc;
use pumpkin_api_macros::{plugin_impl, plugin_method};
use pumpkin::plugin::Context;
use crate::init::initialize_plugin;

#[plugin_method] 
async fn on_load(&mut self, server: Arc<Context>) -> Result<(), String> {
    initialize_plugin(server).await
}

#[plugin_impl]
pub struct MyPlugin {}

impl MyPlugin {
    pub fn new() -> Self {
        MyPlugin {}
    }
}

impl Default for MyPlugin {
    fn default() -> Self {
        Self::new()
    }
}
