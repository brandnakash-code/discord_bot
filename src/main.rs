use serenity::prelude::*;
use std::env;
use std::sync::Mutex;

struct Handler {
    prefix: Mutex<char>,
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: serenity::model::channel::Message) {
        if msg.author.bot {
            return;
        }

        let prefix = self.prefix.lock().unwrap();

        if msg.content.starts_with(*prefix) {
            let _ = msg.channel_id.say(&ctx.http, "No commands yet!");
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN environment variable is missing");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler {
            prefix: Mutex::new('!'),
        })
        .await
        .expect("Error creating client");

    if let Err(error) = client.start().await {
        eprintln!("Client error: {error}");
    }
}
