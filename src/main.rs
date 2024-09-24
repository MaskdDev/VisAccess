use poise::serenity_prelude as serenity;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;
use std::ops::Not;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // On ready event
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    // On message event
    async fn message(&self, _ctx: Context, msg: Message) {
        if msg.attachments.len() > 0 {
            for attachment in msg.attachments.clone() {
                // Get content type
                let content_type = attachment.content_type.unwrap_or("none".to_string());

                // Check attachment type
                if content_type.starts_with("image") && content_type.starts_with("image/gif").not()
                {
                    match attachment.description {
                        Some(_) => {}
                        None => {
                            // Check for ID in text
                            if msg.content.to_ascii_lowercase().contains("id").not() {
                                msg.reply(
                                    _ctx.http(),
                                    "You haven't added an image ID to this image! For more information on image and video IDs, check out this message in our rules: https://discord.com/channels/1247088652656312360/1247088653558353963/1262346544603201557.",
                                )
                                .await
                                .expect("Error replying to message.");
                            }
                        }
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Get token from environment
    let token = env::var("token").expect("Expected a token in the environment");

    // Set intents
    let intents = GatewayIntents::all();

    // Build client
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client.");

    // Set messages cache
    client.cache.set_max_messages(100);

    // Start client
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
