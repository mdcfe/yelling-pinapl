use serenity::model::prelude::{ MessageId, ChannelId };
use chrono::{DateTime, Utc};

pub struct Reminder {
    pub reply_to: MessageId,
    pub reply_channel: ChannelId,
    pub time: DateTime<Utc>,
    pub message: Option<String>
}

impl Reminder {
    pub fn new(reply_to: MessageId, reply_channel: ChannelId, time: DateTime<Utc>, message: Option<String>) -> Reminder {
        Reminder {
            reply_to,
            reply_channel,
            time,
            message
        }
    }
}

impl Clone for Reminder {
    fn clone(&self) -> Self {
        Reminder::new(self.reply_to, self.reply_channel, self.time, self.message.clone())
    }
}
