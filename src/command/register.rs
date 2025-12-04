use async_trait::async_trait;
use pumpkin::{
    command::{ 
        args::ConsumedArgs, dispatcher::CommandError, tree::builder::literal, tree::CommandTree,
        CommandExecutor, CommandSender,
    },
    plugin::{player::player_join::PlayerJoinEvent, Context, EventHandler, EventPriority},
    server::Server,
};

pub struct Register; 

#[async_trait] 
impl CommandExecutor for Register {
    async fn execute<'a>(
        &self,
        _sender: &mut CommandSender,
        _: &Server,
        _: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        println!("登录成功");
        Ok(())
    }
}