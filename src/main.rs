mod structs;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::prelude::{Message, UserId, MessageReference, ReactionType};
use serenity::framework::standard::{
    StandardFramework,
    help_commands,
    Args,
    HelpOptions,
    CommandGroup,
    CommandResult,
    macros::{
        command,
        group,
        help
    }
};

use std::env;
use std::collections::HashSet;
use std::time::{Duration, UNIX_EPOCH, SystemTime};

use structs::Reminder;

#[group]
#[commands(ping)]
#[commands(remind)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!?!")) // set the bot's prefix to "~"
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

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "hello world").await?;

    Ok(())
}

#[command]
#[description = "Set a reminder."]
#[aliases("reminder", "remindme")]
#[usage("remind <delay> [message]")]
async fn remind(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // parse delay string -> duration
    // if not valid, error
    if args.is_empty() {
        msg.reply(ctx, "You need to provide a delay! Example: `2d4h10m`").await?;
        return Ok(());
    }

    // HELP wtf is going on here
    let duration_arg = args.single::<String>().unwrap();
    let duration = match parse_duration::parse(duration_arg.as_ref()) {
        Ok(d) => d,
        Err(_e) => {
            msg.reply(ctx, format!("`{}` is not a valid delay.", duration_arg)).await?;
            return Ok(());
        }
    };

    // HELP this seems... meh
    let message = args.remains().map(|s| s.parse().unwrap());
    let reminder = Reminder {
        reply_to: msg.id,
        reply_channel: msg.channel_id,
        time: msg.timestamp + chrono::Duration::from_std(duration)?,
        message
    };

    // todo store Reminder on disk in case of restart

    schedule_reminder(ctx, reminder.clone());

    println!("reply_to: {} ; reply_channel: {} ; time: {} ; message: {}",
             reminder.reply_to,
             reminder.reply_channel,
             reminder.time,
             reminder.message.unwrap_or_default()
    );

    msg.react(ctx, ReactionType::from('👍')).await?;

    Ok(())
}

fn schedule_reminder(ctx: &Context, reminder: Reminder) {
    let ctx1 = ctx.clone();

    tokio::spawn(async move {
        let reply_channel = reminder.reply_channel;
        let reply_user = reminder.reply_to;

        // HELP is there a better way to do this
        let duration = Duration::from_secs(reminder.time.timestamp() as u64) - SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        tokio::time::sleep(duration).await;

        let text = reminder.message.unwrap_or("No message provided".parse().unwrap());

        let _ = reminder.reply_channel.send_message(&ctx1.http, |m| {
            m.content(format!("Reminder: {}", text));
            m.reference_message(MessageReference::from((reply_channel, reply_user)));
            m.allowed_mentions(|am| {
                am.replied_user(true);

                am
            });

            m
        }).await;
    });
}
