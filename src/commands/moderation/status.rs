use crate::helpers::fetching;
use crate::types::{Context, Error};
use ::serenity::all::Mentionable;
use poise::serenity_prelude as serenity;

/// View the warn status for a user.
#[poise::command(
    slash_command,
    category = "General",
    default_member_permissions = "MANAGE_MESSAGES"
)]
pub async fn status(
    ctx: Context<'_>,
    #[description = "The user you wish to view the warn status for."] user: serenity::User,
) -> Result<(), Error> {
    // Get the user specified
    let warn_user = fetching::fetch_user(&ctx, user.id.get()).await;

    // Create reply
    let reply = poise::CreateReply::default()
        .content(format!(
            "{} currently has {} active warns, and has received {} warns overall.",
            user.mention(),
            warn_user.active_warns,
            warn_user.warns.len()
        ))
        .ephemeral(true);

    // Respond to interaction
    ctx.send(reply).await?;

    // Return Ok
    Ok(())
}
