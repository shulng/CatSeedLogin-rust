use std::sync::Arc;
use async_trait::async_trait;
use pumpkin::plugin::{player::player_join::PlayerJoinEvent, EventHandler};
use pumpkin::server::Server;
use pumpkin_util::text::{color::NamedColor, TextComponent};
use pumpkin_api_macros::with_runtime;

pub struct MyJoinHandler;

#[with_runtime(global)]
#[async_trait]
impl EventHandler<PlayerJoinEvent> for MyJoinHandler {
    async fn handle_blocking(&self, _server: &Arc<Server>, event: &mut PlayerJoinEvent) {
        event.join_message =
            TextComponent::text(format!("欢迎游玩, {}!", event.player.gameprofile.name))
                .color_named(NamedColor::Green);
    }
}
