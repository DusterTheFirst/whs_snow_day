use crate::post::Post;
use crate::webhook::discord::{
    DiscordEmbed, DiscordEmbedAuthor, DiscordWebhook, DiscordWebhookType,
};
use reqwest::Result;

pub fn alert_discord(webhook: &str, post: &Post) -> Result<()> {
    info!(r#"Alerting discord webhook "{}""#, webhook);

    let client = reqwest::Client::new();
    client
        .post(webhook)
        .json(&DiscordWebhook {
            content: Some(format!("@everyone {}", post.title)),
            embeds: Some(vec![DiscordEmbed {
                title: Some(post.title.clone()),
                r#type: DiscordWebhookType::Rich,
                description: Some(format!(
                    "There has been an urgent alert for WHS issued\nFor more information, click the title of the embed or [this link here]({})",
                    post.link
                )),
                url: Some(post.link.clone()),
                timestamp: Some(post.date),
                color: Some(4388222), // #42f57e
                footer: None,
                image: None,
                thumbnail: None,
                author: Some(DiscordEmbedAuthor {
                    name: Some("Zachary Kohnen (DusterTheFirst)".to_owned()),
                    url: Some("https://github.com/dusterthefirst".to_owned()),
                    icon_url: Some("https://avatars0.githubusercontent.com/u/14093962?s=400&u=5ce85f9c1e5883f103ef6756f80a95141e20a1b1&v=4".to_owned())
                }),
                fields: None
            }]),
            username: None,
            avatar_url: None,
            tts: None
        })
        .send()?;

    Ok(())
}
