use poise::serenity_prelude as serenity;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // The message event, run whenever a message is sent.
    async fn message(&self, ctx: Context, message: Message) {
        // Initialise counters for attachments
        let mut no_id = 0;
        let mut total_id = 0;

        // Iterate over all attachments
        for attachment in message.attachments.clone() {
            // Get content type
            let content_type = attachment.content_type.unwrap_or("none".to_string());

            // Check if the attachment is an image, but not a gif
            if content_type.starts_with("image") && !content_type.starts_with("image/gif") {
                // Increment total ID-able
                total_id += 1;

                // Check for the lack of an ID
                if !(attachment.description.is_some()
                    || message.content.to_ascii_lowercase().contains("id:"))
                {
                    no_id += 1;
                }
            }
        }

        // Check if any attachments don't have an ID
        if no_id > 0 {
            // Pick message based on the number of attachments without an ID
            let reply = if no_id == total_id {
                "You haven't added image IDs to any of these images! For more information on image and video IDs, check out this message in our rules: https://discord.com/channels/1247088652656312360/1247088653558353963/1262346544603201557.".to_string()
            } else if no_id == 1 {
                "You haven't added an image ID to this image! For more information on image and video IDs, check out this message in our rules: https://discord.com/channels/1247088652656312360/1247088653558353963/1262346544603201557.".to_string()
            } else {
                format!("You haven't added an image ID to {} of these images! For more information on image and video IDs, check out this message in our rules: https://discord.com/channels/1247088652656312360/1247088653558353963/1262346544603201557.", no_id)
            };

            // Send message in reply to the offending message
            message.reply(ctx, reply).await.expect(&format!(
                "Could not reply to a message with ID: {}.",
                message.id
            ));
        }
    }

    // On ready event
    async fn ready(&self, ctx: Context, ready: Ready) {
        // Send connected message
        println!("{} is connected!", ready.user.name);

        // Set bot presence
        ctx.shard.set_presence(
            Some(serenity::ActivityData::listening("Image IDs!")),
            serenity::OnlineStatus::Idle,
        );
    }
}

#[tokio::main]
async fn main() {
    // Get token from environment
    let token = env::var("token").expect("Expected a token in the environment.");

    // Set intents
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    // Build client
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client.");

    // Start client
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
