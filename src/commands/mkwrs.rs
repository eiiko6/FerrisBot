use crate::{Context, Error};
use chrono::Local;
use mkwrs_scraper::fetch_today_records;

/// Send the list of mkwrs records
#[poise::command(slash_command)]
pub async fn mkwrs(ctx: Context<'_>) -> Result<(), Error> {
    let now = Local::now().format("%Y-%m-%d").to_string();

    match fetch_today_records(&now).await {
        Ok(result) => {
            ctx.say(
                result
                    .iter()
                    .map(|r| format!("{} - {} (<{}>)", r.track, r.time, r.video_link))
                    .collect::<Vec<_>>()
                    .join("\n"),
            )
            .await?
        }
        Err(_) => ctx.say("Failed to evaluate expression!").await?,
    };
    Ok(())
}
