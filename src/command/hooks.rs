use log::{debug};
use serenity::framework::standard::macros::hook;
use serenity::client::Context;
use serenity::model::prelude::Message;
use serenity::framework::standard::DispatchError;

#[hook]
pub(crate) async fn unknown_command(ctx: &Context, msg: &Message, unknown_command_name: &str) {
    let from: String = match msg.guild_id {
        Some(guild_id) => guild_id.0.to_string(),
        None => "DM".to_string(),
    };
    debug!("Command '{}' not found from '{}'@'{}' ", unknown_command_name, from, msg.author.name);

    let _ = msg.channel_id.say(&ctx.http, &format!("Could not find command named '{}'", unknown_command_name)).await;
}

#[hook]
pub(crate) async fn delay_action(ctx: &Context, msg: &Message) {
    // You may want to handle a Discord rate limit if this fails.
    let _ = msg.react(ctx, '⏱').await;
}

#[hook]
pub(crate) async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    if let DispatchError::Ratelimited(info) = error {
        // We notify them only once.
        if info.is_first_try {
            let _ = msg
                .channel_id
                .say(&ctx.http, &format!("Try this again in {} seconds.", info.as_secs()))
                .await;
        }
    }
}