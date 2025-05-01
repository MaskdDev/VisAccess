use poise::serenity_prelude as serenity;
use serenity::prelude::*;
use std::env;

// Import modules
mod commands;
mod database;
mod helpers;
mod listeners;
mod structs;
mod types;

// Re-export types
pub use database::Database;
pub use types::{ClientData, Context, Error, PoiseCommand};

#[tokio::main]
async fn main() {
    // Get token from environment
    let token = env::var("token").expect("Expected a token in the environment.");

    // Set intents
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    // Create bot framework
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::get_commands(),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands)
                    .await
                    .expect("Error registering commands.");
                Ok(ClientData {})
            })
        })
        .build();

    // Create serenity client
    let mut client = serenity::ClientBuilder::new(token, intents)
        .event_handler(listeners::Handler)
        .framework(framework)
        .await
        .expect("Error creating client.");

    // Get Redis URL
    let redis_url = env::var("redis_url").expect("Expected a Redis URL in the environment.");

    // Open Redis client
    let redis_client = redis::Client::open(redis_url).expect("Error connecting to Redis.");

    // Open Redis connection
    let connection = redis_client
        .get_multiplexed_async_connection()
        .await
        .expect("Error creating Redis connection.");

    // Create database
    let database = Database::from_connection(connection);

    // Move database into client data
    client.data.write().await.insert::<Database>(database);

    // Start client
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
