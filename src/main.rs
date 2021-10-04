use std::collections::HashSet;
use log::info;
use anyhow::{Result, bail};

use serenity::client::bridge::gateway::GatewayIntents;
use serenity::framework::StandardFramework;
use serenity::framework::standard::buckets::LimitedFor;
use serenity::http::Http;
use serenity::model::gateway::Ready;
use serenity::{async_trait, prelude::*};

mod config;
mod command;

use crate::config::Config;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load_config()?;

    tracing_subscriber::fmt().init();

    let http = Http::new_with_token(&config.discord.token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
    .configure(|c| c
               .with_whitespace(true)
               .on_mention(Some(bot_id))
               .prefix("!")
               .owners(owners)
    )
    .unrecognised_command(command::hooks::unknown_command)
    .on_dispatch_error(command::hooks::dispatch_error)
    .bucket("general", |b| b.delay(1)).await
    .bucket("music", |b| b.limit(2).time_span(30).delay(5)
        .limit_for(LimitedFor::Guild)
        .await_ratelimits(1)).await
        .help(&command::help::HELP)
        .group(&command::utils::UTILS_GROUP);

    let mut client =
        Client::builder(&config.discord.token)
            .event_handler(Handler)
            .framework(framework)
            .intents(GatewayIntents::all())
            .await.expect("Err creating client");

    if let Err(why) = client.start().await {
        bail!("Client error: {:?}", why);
    }

    Ok(())
}