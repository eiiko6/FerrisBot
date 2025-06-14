use crate::{Context, Error};
use mkwrs_scraper::fetch_today_records;

/// Send the list of mkwrs records
#[poise::command(slash_command)]
pub async fn mkwrs(ctx: Context<'_>) -> Result<(), Error> {
    match fetch_today_records("2025-06-14").await {
        Ok(result) => {
            ctx.say(format!(
                "Result: {}",
                result
                    .iter()
                    .map(|r| r.to_string())
                    .collect::<Vec<_>>()
                    .join("\n")
            ))
            .await?
        }
        Err(_) => ctx.say("Failed to evaluate expression!").await?,
    };
    Ok(())
}
