use crate::{
    error::Result,
    http::HttpClient,
    models::{guild::GuildMember, user::User},
};

use super::{Embed, MentionChannel, MessageReference, Attachment, Reaction, MessageApplication};

use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Debug, Deserialize, Serialize)]
/// Represents a message sent in a channel within Discord.
/// [Discord Documentation](https://discord.com/developers/docs/resources/channel#message-object)
pub struct Message {
    /// ID of the message
    pub id: String,
    
    /// ID of the channel the message was sent in
    pub channel_id: String,

    /// ID of the guild the message was sent in case it was sent in one
    pub guild_id: Option<String>,

    /// Author of the message
    pub author: User,

    /// Member properties for this message's author in case it was sent in a guild
    pub member: Option<GuildMember>,

    /// Contents of the message
    pub content: String,

    /// When this message was sent, as string
    pub timestamp: String,

    /// When this message was edited (or `None` if never)
    pub edited_timestamp: Option<String>,

    /// Whether this was a TTS message
    pub tts: bool,

    /// Whether this message mentions everyone
    pub mention_everyone: bool,

    /// Users specifically mentioned in the message
    pub mentions: Vec<User>,

    /// Roles specifically mentioned in this message
    pub mention_roles: Vec<String>,
    
    /// Channels specifically mentioned in this message
    #[serde(default)]
    pub mentions_channels: Vec<MentionChannel>,

    /// Any attached files
    pub attachments: Vec<Attachment>,

    /// Any embedded content
    #[serde(default)]
    pub embed: Vec<Embed>,

    /// Reactions to the message
    #[serde(default)]
    pub reactions: Vec<Reaction>,

    /// Used for validating a message was sent
    pub nonce: Option<String>,

    /// Whether this message is pinned
    pub pinned: bool,

    /// If the message is generated by a webhook, this is the webhook's id
    pub webhook_id: Option<String>,

    /// Type of message
    #[serde(rename = "type")]
    pub kind: Option<MessageKind>,

    // activity: MessageActivity,

    /// Sent with Rich Presence-related chat embeds
    pub application: Option<MessageApplication>,

    /// Reference data sent with crossposted messages
    pub message_reference: Option<MessageReference>,

    /// Message flags ORd together, describes extra features of the message
    pub flags: Option<u64>,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum MessageKind {
    Regular = 0,
    RecipientAdd = 1,
    RecipientRemove = 2,
    Call = 3,
    ChannelNameChange = 4,
    ChannelIconChange = 5,
    ChannelPinnedMessage = 6,
    GuildMemberJoin = 7,
    UserPremiumGuildSub = 8,
    UserPremiumGuildSubT1 = 9,
    UserPremiumGuildSubT2 = 10,
    UserPremiumGuildSubT3 = 11,
    ChannelFollowAdd = 12,
    GuildDiscoveryDisqualified = 14,
    GuildDiscoveryRequalified = 15
}

impl Message {

    /// Shortcut for [`HttpClient.send_message`]
    ///
    /// [`HttpClient.send_message`]: ../../../struct.HttpClient.html#method.send_message
    pub async fn send(&self, http: &HttpClient, content: impl AsRef<str>) -> Result<Message> {
        http.send_message(&self.channel_id, content).await
    }

    /// Shortcut for [`HttpClient.send_embed`]
    ///
    /// [`HttpClient.send_embed`]: ../../../struct.HttpClient.html#method.send_embed
    pub async fn send_embed(&self, http: &HttpClient, embed: super::Embed) -> Result<Message> {
        http.send_embed(&self.channel_id, embed).await
    }

    /// Shortcut for [`HttpClient.add_reaction`]
    ///
    /// [`HttpClient.add_reaction`]: ../../../struct.HttpClient.html#method.add_reaction
    pub async fn add_reaction(&self, http: &HttpClient, emoji: impl AsRef<str>) -> Result<()> {
        http.add_reaction(&self.channel_id, &self.id, emoji).await
    }

    /// Shortcut for [`HttpClient.add_reaction`]
    ///
    /// [`HttpClient.add_reaction`]: ../../../struct.HttpClient.html#method.add_reaction
    pub async fn add_reaction_to_message(&self, http: &HttpClient, message_id: impl AsRef<str>, emoji: impl AsRef<str>) -> Result<()> {
        http.add_reaction(&self.channel_id, message_id, emoji).await
    }

    /// Shortcut for [`HttpClient.delete_message`]
    ///
    /// [`HttpClient.delete_message`]: ../../../struct.HttpClient.html#method.delete_message
    pub async fn remove(&self, http: &HttpClient) -> Result<()> {
        http.delete_message(&self.channel_id, &self.id).await
    }

    /// Shortcut for [`HttpClient.pin_message`]
    ///
    /// [`HttpClient.pin_message`]: ../../../struct.HttpClient.html#method.pin_message
    pub async fn pin(&self, http: &HttpClient) -> Result<()> {
        http.pin_message(&self.channel_id, &self.id).await
    }

    /// Shortcut for [`HttpClient.unpin_message`]
    ///
    /// [`HttpClient.unpin_message`]: ../../../struct.HttpClient.html#method.unpin_message
    pub async fn unpin(&self, http: &HttpClient) -> Result<()> {
        http.unpin_message(&self.channel_id, &self.id).await
    }
}
