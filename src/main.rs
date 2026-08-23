use phf::phf_map;
use serenity::prelude::*;
use std::env;
use std::sync::Mutex;

static HELP: phf::Map<&'static str, &'static str> = phf_map! {
    "prefix" => "A prefix is an indicator to the bot that you are entering a command. For every command, enter a prefix before it. By default, the prefix to a command is '!' (e.g. `!command`).\nUse `prefix set {new prefix}` to change the prefix.\nUse `prefix default` to reset the prefix.\nUse `prefix help` to show this message.",
    "US" => "United States",
};

struct Handler {
    prefix: Mutex<String>,
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: serenity::model::channel::Message) {
        if msg.author.bot {
            return;
        }

        let prefix: String = self.prefix.lock().unwrap().to_string();

        if !msg.content.starts_with(&prefix) {
            return;
        }

        let cmd = Command {
            text: msg.content.strip_prefix(&prefix).unwrap(),
            parts: msg
                .content
                .strip_prefix(&prefix)
                .unwrap()
                .split_whitespace()
                .collect(),
        };

        if cmd.parts.is_empty() {
            return;
        }

        match cmd.parts.first().copied() {
            Some(command) if command.eq_ignore_ascii_case("echo") => {
                say(&msg, &ctx, cmd.strip_sub(1)).await;
            }
            Some(command) if command.eq_ignore_ascii_case("prefix") => {
                match cmd.parts.get(1).copied() {
                    Some(action) if action.eq_ignore_ascii_case("set") => {
                        let new_prefix = cmd.strip_sub(2);
                        let new_prefix = if new_prefix.is_empty() {
                            "!"
                        } else {
                            new_prefix
                        };
                        let response = format!(
                            "Set prefix to '{}'. To enter a command, now type '{}command'",
                            new_prefix, new_prefix
                        );

                        {
                            let mut current_prefix = self.prefix.lock().unwrap();
                            *current_prefix = new_prefix.to_string();
                        }

                        say(&msg, &ctx, &response).await;
                    }
                    Some(action) if action.eq_ignore_ascii_case("default") => {
                        let response = "Set prefix to '!'. To enter a command, now type '!command'";

                        {
                            let mut current_prefix = self.prefix.lock().unwrap();
                            *current_prefix = "!".to_string();
                        }

                        say(&msg, &ctx, response).await;
                    }
                    Some(action) if action.eq_ignore_ascii_case("help") => {
                        say(&msg, &ctx, HELP["prefix"]).await;
                    }
                    Some(_) | None => {
                        let response: String = format!(
                            "Invalid prefix command. Use '{}prefix help' for help.",
                            prefix
                        );
                        say(&msg, &ctx, &response).await;
                    }
                }
            }
            Some(command) if command.eq_ignore_ascii_case("ping") => {
                say(&msg, &ctx, "pong").await
            }
            _ => {
                say(&msg, &ctx, "Unrecognized command").await;
            }
        }
    }
}

struct Command<'a> {
    text: &'a str,
    parts: Vec<&'a str>,
}

impl<'a> Command<'a> {
    fn strip_sub(&self, amount: usize) -> &'a str {
        if amount == 0 {
            return self.text;
        }

        let mut rest = self.text.trim_start();

        for _ in 0..amount {
            let Some((index, _)) = rest
                .char_indices()
                .find(|(_, character)| character.is_whitespace())
            else {
                return "";
            };

            rest = rest[index..].trim_start();
        }

        rest
    }
}

async fn say(msg: &serenity::model::channel::Message, ctx: &Context, content: &str) {
    let _ = msg.channel_id.say(&ctx.http, content).await;
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN environment variable is missing");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler {
            prefix: Mutex::new(String::from("!")),
        })
        .await
        .expect("Error creating client");

    if let Err(error) = client.start().await {
        eprintln!("Client error: {error}");
    }
}
