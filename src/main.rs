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
        // Initialise counters for images
        let mut id_missing = 0;
        let mut total_images = 0;

        // Iterate over all attachments
        for attachment in message.attachments.clone() {
            // Get content type
            let content_type = attachment.content_type.unwrap_or("none".to_string());

            // Check if the attachment is an image, but not a gif
            if content_type.starts_with("image") && !content_type.starts_with("image/gif") {
                // Increment total ID-able
                total_images += 1;

                // Check for the lack of an ID
                if !(attachment.description.is_some()
                    || message.content.to_ascii_lowercase().contains("id:"))
                {
                    id_missing += 1;
                }
            }
        }

        // Check if any attachments don't have an ID
        if id_missing > 0 {
            // Pick message based on the number of attachments without an ID
            let reply = if id_missing == 1 {
                env::var("SINGULAR_MESSAGE")
                    .expect("You haven't added an image description for this image!")
            } else if id_missing == total_images {
                env::var("ALL_MESSAGE")
                    .expect("You haven't added an image description for any of these images!")
            } else {
                env::var("SOME_MESSAGE")
                    .expect("You haven't added an image description to %n of these images!")
                    .replace("%n", &id_missing.to_string())
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
    let token = env::var("TOKEN").expect("Expected a token in the environment.");

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
