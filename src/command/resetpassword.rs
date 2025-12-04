use async_trait::async_trait;
use pumpkin::{
    command::{ 
        args::ConsumedArgs, dispatcher::CommandError, tree::builder::literal, tree::CommandTree,
        CommandExecutor, CommandSender,
    },
    plugin::{player::player_join::PlayerJoinEvent, Context, EventHandler, EventPriority},
    server::Server,
};

pub struct Resetpassword; 

#[async_trait] 
impl CommandExecutor for Resetpassword {
    async fn execute<'a>(
        &self,
        _sender: &mut CommandSender,
        _: &Server,
        _: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        Ok(())
    }
}