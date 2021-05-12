use serenity::{
    client::Context,
    model::prelude::{
        Message
    },
    framework::standard::{
        CommandResult,
        macros::{
            command
        }
    }
};

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Hello world!").await?;
    Ok(())
}
