use poise::serenity_prelude as serenity;

pub async fn handle_ready(ctx: serenity::Context, ready_event: serenity::Ready) {
    // Display connected message
    println!("{} is connected!", ready_event.user.name);

    // Set bot presence
    ctx.shard.set_presence(
        Some(serenity::ActivityData::listening("Image IDs!")),
        serenity::OnlineStatus::Idle,
    );
}
