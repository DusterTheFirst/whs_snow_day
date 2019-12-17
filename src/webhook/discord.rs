use chrono::naive::NaiveDateTime;
use serde::Serialize;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Serialize)]
pub struct DiscordWebhook {
    /// the message contents (up to 2000 characters)
    pub content: Option<String>,
    /// override the default username of the webhook
    pub username: Option<String>,
    /// override the default avatar of the webhook
    pub avatar_url: Option<String>,
    /// true if this is a TTS message
    pub tts: Option<bool>,
    /// array of up to 10 embed objects	embedded rich content
    pub embeds: Option<Vec<DiscordEmbed>>,
}

#[derive(Debug, Serialize)]
pub struct DiscordEmbed {
    /// title of embed
    pub title: Option<String>,
    /// type of embed (always "rich" for webhook embeds)
    pub r#type: DiscordWebhookType,
    /// description of embed
    pub description: Option<String>,
    /// url of embed
    pub url: Option<String>,
    /// ISO8601 timestamp of embed content
    pub timestamp: Option<NaiveDateTime>,
    /// integer	color code of the embed
    pub color: Option<u32>,
    /// footer information
    pub footer: Option<DiscordEmbedFooter>,
    /// image information
    pub image: Option<DiscordEmbedImage>,
    /// thumbnail information
    pub thumbnail: Option<DiscordEmbedThumbnail>,
    /// author information
    pub author: Option<DiscordEmbedAuthor>,
    /// fields information
    pub fields: Option<Vec<DiscordEmbedField>>,
}

#[derive(Debug, Serialize)]
pub struct DiscordEmbedFooter {
    /// footer text
    pub text: String,
    /// url of footer icon (only supports http(s) and attachments)
    pub icon_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DiscordEmbedImage {
    /// source url of image (only supports http(s) and attachments)
    pub url: Option<String>,
    /// height of image
    pub height: Option<u32>,
    /// width of image
    pub width: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct DiscordEmbedThumbnail {
    /// source url of thumbnail (only supports http(s) and attachments)
    pub url: Option<String>,
    /// height of thumbnail
    pub height: Option<u32>,
    /// width of thumbnail
    pub width: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct DiscordEmbedAuthor {
    /// name of author
    pub name: Option<String>,
    /// url of author
    pub url: Option<String>,
    /// url of author icon (only supports http(s) and attachments)
    pub icon_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DiscordEmbedField {
    /// name of the field
    pub name: String,
    /// value of the field
    pub value: String,
    /// whether or not this field should display inline
    pub inline: Option<bool>,
}

#[derive(Debug, Serialize)]
pub enum DiscordWebhookType {
    Rich,
}

impl Default for DiscordWebhookType {
    fn default() -> Self {
        Self::Rich
    }
}

impl Display for DiscordWebhookType {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::Rich => write!(f, "rich"),
        }
    }
}
