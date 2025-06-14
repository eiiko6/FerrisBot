use crate::Context;
use poise::serenity_prelude as serenity;
use std::time::Duration;
use tokio::time::{Instant as TokioInstant, interval_at};

pub async fn schedule_daily_dm(
    ctx: Context<'_>,
    user_id: serenity::UserId,
    hour: u32,
    minute: u32,
) {
    let now = chrono::Local::now();
    let today = now.date_naive();

    // Get target time today, or tomorrow if today target time passed
    let target_time_today = today.and_hms_opt(hour, minute, 0).unwrap();

    let target_time = if chrono::Local::now().time() < target_time_today.time() {
        target_time_today
    } else {
        today
            .succ_opt()
            .unwrap()
            .and_hms_opt(hour, minute, 0)
            .unwrap()
    };

    let now_naive = now.naive_local();
    let duration_until_target = (target_time - now_naive)
        .to_std()
        .unwrap_or_else(|_| Duration::from_secs(0)); // fallback zero if negative

    tokio::time::sleep(duration_until_target).await;

    let mut interval = interval_at(TokioInstant::now(), Duration::from_secs(60 * 60 * 24));

    loop {
        interval.tick().await;
        if let Ok(dm_channel) = user_id
            .create_dm_channel(&ctx.serenity_context().http)
            .await
        {
            let _ = dm_channel
                .say(&ctx.serenity_context().http, "Daily message!")
                .await;
        }
    }
}
