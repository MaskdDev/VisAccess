use poise::serenity_prelude as serenity;
use serenity::all::EventHandler;
use serenity::{async_trait, Context, Message, Ready};

// Import modules
pub mod message;
pub mod ready;

// Create event handler
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // The on-ready event, run when a shard is ready and the READY payload is sent by discord.
    async fn ready(&self, ctx: Context, ready_event: Ready) {
        ready::handle_ready(ctx, ready_event).await;
    }

    // The on message event, run when a message is sent.
    async fn message(&self, ctx: Context, message: Message) {
        message::handle_message(ctx, message).await;
    }
}
