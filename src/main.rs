mod commands;
mod structs;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::prelude::{Message, UserId};
use serenity::framework::standard::{
    StandardFramework,
    help_commands,
    Args,
    HelpOptions,
    CommandGroup,
    CommandResult,
    macros::{
        group,
        help
    }
};

use std::env;
use std::collections::HashSet;

use commands::{
    misc::*,
    reminders::*
};

#[group]
#[commands(ping, remind)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!?!"))
        .group(&GENERAL_GROUP)
        .help(&HELP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[help]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
