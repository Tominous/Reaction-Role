mod utils;

use std::env;

use serenity::{
    async_trait,
    client::bridge::gateway::GatewayIntents,
    model::{channel::{Message, Reaction}, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, msg: Message) {
        println!("Received message: {}", msg.content);
    }

    async fn reaction_add(&self, _ctx: Context, _add_reaction: Reaction) {
        if let Some(user_id) = _add_reaction.user_id {
            println!("{} set {} in {}", user_id, _add_reaction.emoji, _add_reaction.message_id)
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Successfully started {}", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("BOT_TOKEN").expect("Expected a discord bot token in the environment.");

    println!("{}", token);

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .intents(GatewayIntents::GUILDS | GatewayIntents::GUILD_EMOJIS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILD_MESSAGE_REACTIONS)
        .await
        .expect("An exception occurred while trying to create the discord client.");

    if let Err(error) = client.start().await {
        println!("Client error: {}", error);
    }
}
