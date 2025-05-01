use crate::structs::user::User;
use crate::{Context, Database};
use poise::serenity_prelude as serenity;
use serenity::Context as SerenityContext;

/// Fetch a user using serenity context.
pub async fn fetch_user_serenity(ctx: &SerenityContext, user_id: u64) -> User {
    // Get client data read
    let client_data = ctx.data.read().await;

    // Get database from client data
    let database = client_data
        .get::<Database>()
        .expect("Could not get database from client data.");

    // Get and return user
    database.get_user(user_id).await
}

/// Fetch a user using poise context.
pub async fn fetch_user(ctx: &Context<'_>, user_id: u64) -> User {
    // Return user using serenity context
    fetch_user_serenity(ctx.serenity_context(), user_id).await
}
