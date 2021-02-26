#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::env;

use serenity::{
    async_trait,
    client::bridge::gateway::GatewayIntents,
    model::{channel::{Message, Reaction}, gateway::Ready},
    prelude::*,
};

use crate::modules::reaction::handle_reaction_event;

mod modules;
mod core;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, msg: Message) {
        debug!("Received message: {}", msg.content);
    }

    async fn reaction_add(&self, _ctx: Context, _add_reaction: Reaction) {
        handle_reaction_event(_ctx, _add_reaction).await;
    }

    async fn reaction_remove(&self, _ctx: Context, _removed_reaction: Reaction) {
        handle_reaction_event(_ctx, _removed_reaction).await;
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("Successfully started reaction role bot on client '{}#{}'", ready.user.name, ready.user.discriminator);
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    debug!("Fetching bot token from environment");
    let token = env::var("BOT_TOKEN").expect("Expected a discord bot token in the environment.");
    debug!("Successfully fetched bot token from the environment");

    debug!("Creating discord client");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .intents(GatewayIntents::GUILDS | GatewayIntents::GUILD_EMOJIS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILD_MESSAGE_REACTIONS)
        .await
        .expect("An exception occurred while trying to create the discord client.");
    debug!("Successfully created a discord bot client");

    if let Err(error) = client.start().await {
        error!("Client error: {}", error);
    }
}