use serenity::framework::standard::macros::{command, group};
use serenity::client::Context;
use serenity::model::prelude::Message;
use serenity::framework::standard::CommandResult;

#[group]
#[commands(ping, stats)]
struct Utils;

#[command]
#[bucket = "general"]
#[only_in(guilds)]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
#[bucket = "general"]
#[owners_only]
async fn stats(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "TODO stats!").await?;

    Ok(())
}