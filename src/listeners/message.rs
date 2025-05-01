use poise::serenity_prelude as serenity;
use serenity::Message;

pub async fn handle_message(ctx: serenity::Context, msg: Message) {
    if msg.attachments.len() > 0 {
        for attachment in msg.attachments.clone() {
            // Get content type
            let content_type = attachment.content_type.unwrap_or("none".to_string());

            // Check attachment type
            if content_type.starts_with("image") && !content_type.starts_with("image/gif") {
                match attachment.description {
                    Some(_) => {}
                    None => {
                        // Check for ID in text
                        if !msg.content.to_ascii_lowercase().contains("id:") {
                            msg.reply(
                                    &ctx,
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
