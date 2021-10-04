use std::collections::HashSet;

use serenity::client::Context;
use serenity::framework::standard::{
    CommandGroup,
    CommandResult,
    help_commands,
    HelpOptions,
    Args,
    macros::help,
};
use serenity::model::{
    channel::Message,
    prelude::UserId
};

#[help]
#[individual_command_tip = "Hello!\n\nIf you want more information about a specific command, just pass the command as argument."]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
#[indention_prefix = "\n"]
//#[lacking_permissions = "Hide"]
//#[lacking_role = "Nothing"]
#[wrong_channel = "Strike"]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}