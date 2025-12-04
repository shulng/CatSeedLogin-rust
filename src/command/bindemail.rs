use async_trait::async_trait;
use pumpkin::{
    command::{ 
        args::ConsumedArgs, dispatcher::CommandError, tree::builder::literal, tree::CommandTree,
        CommandExecutor, CommandSender,
    },
    plugin::{player::player_join::PlayerJoinEvent, Context, EventHandler, EventPriority},
    server::Server,
};

pub struct Bindemail; 

#[async_trait] 
impl CommandExecutor for Bindemail {
    async fn execute<'a>(
        &self,
        _sender: &mut CommandSender,
        _: &Server,
        _: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        Ok(())
    }
}