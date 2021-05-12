use serenity::{
    client::Context,
    model::prelude::{
        Message,
        MessageReference,
        ReactionType
    },
    framework::standard::{
        Args,
        CommandResult,
        macros::{
            command
        }
    }
};

use crate::structs::Reminder;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[command]
#[description = "Set a reminder."]
#[aliases("reminder", "remindme")]
#[usage("remind <delay> [message]")]
pub async fn remind(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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
