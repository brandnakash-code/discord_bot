use serenity::prelude::*;
use std::env;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: serenity::model::channel::Message) {
        if msg.content.eq_ignore_ascii_case("!ping") {
            let _ = msg.channel_id.say(&ctx.http, "Pong!").await;
        }
    }

    async fn ready(&self, _: Context, ready: serenity::model::gateway::Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("DISCORD_TOKEN environment variable is missing");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(error) = client.start().await {
        eprintln!("Client error: {error}");
    }
}