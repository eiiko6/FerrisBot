use crate::utils::user_cache::UserCache;
use crate::{Context, Error};
use chrono::Local;
use mkwrs_scraper::fetch_records;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Deserialize, Clone, Hash, Eq, Serialize, PartialEq)]
pub struct Record {
    track: String,
    time: String,
}

/// See mkwrs records
#[poise::command(slash_command, subcommands("help", "today", "all", "reset"))]
pub async fn mkwrs(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Send the usage of the command
#[poise::command(slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("
        ## mkwrs usage:\n
        - `mkwrs help` -> Show this message
        - `mkwrs today` -> Show records made today if any
        - `mkwrs all` -> Show the record for every track, highlighting those that the user has not seen yet
        - `mkwrs reset` -> Reset the seen records for the user
        ").await?;

    Ok(())
}

/// Send the list of mkwrs records that have been added today
#[poise::command(slash_command)]
pub async fn today(ctx: Context<'_>) -> Result<(), Error> {
    let now = Local::now().format("%Y-%m-%d").to_string();

    match fetch_records(&now).await {
        Ok(result) => {
            if !result.is_empty() {
                ctx.say(
                    result
                        .iter()
                        .map(|r| format!("{} - {} (<{}>)", r.track, r.time, r.video_link))
                        .collect::<Vec<_>>()
                        .join("\n"),
                )
                .await?
            } else {
                ctx.say("No records for today.").await?
            }
        }
        Err(_) => ctx.say("Failed to retrieve records!").await?,
    };

    Ok(())
}

/// Send the list of the mkwrs record for each track
#[poise::command(slash_command)]
pub async fn all(ctx: Context<'_>) -> Result<(), Error> {
    let user_id = ctx.author().id.to_string();

    let mut cache = UserCache::load("user_cache.json").unwrap_or_default();

    let seen = cache.get_seen_tracks(&user_id);
    let mut new_seen = HashSet::new();

    let mut seen_hashmap = HashMap::new();
    for track in seen.iter() {
        seen_hashmap.insert(track.track.clone(), track);
    }

    let records = match fetch_records("").await {
        Ok(result) => result
            .iter()
            .map(|r| {
                let rec = Record {
                    track: r.track.clone(),
                    time: r.time.clone(),
                };

                let time_diff = if let Some(old) = seen_hashmap.get(&rec.track) {
                    if old.time != r.time {
                        if let (Some(old_sec), Some(new_sec)) =
                            (parse_time(&old.time), parse_time(&r.time))
                        {
                            let d = old_sec - new_sec;
                            if d > 0.0 {
                                format!(" (⬇️ -{:.3}s)", d)
                            } else if d < 0.0 {
                                format!(" (⬆️ +{:.3}s)", d.abs())
                            } else {
                                String::new()
                            }
                        } else {
                            String::new()
                        }
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                };

                let video = match r.video_link.is_empty() {
                    true => "No video available.".to_string(),
                    false => format!("<{}>", r.video_link),
                };

                let decorated = if seen.contains(&rec) {
                    format!("{} - {} -> {}", r.track, r.time, video)
                } else {
                    format!("⚡ **{} - {} {} -> {}**", r.track, r.time, time_diff, video)
                };
                new_seen.insert(rec);
                decorated
            })
            .collect::<Vec<_>>(),
        Err(_) => {
            ctx.say("Failed to retrieve records!").await?;
            return Ok(());
        }
    };

    if records.is_empty() {
        ctx.say("No records found.").await?;
    } else {
        // Send in chunks due to 2000 char Discord limit
        let mut current_chunk = String::new();
        for line in records {
            if current_chunk.len() + line.len() > 1800 {
                ctx.say(current_chunk).await?;
                current_chunk = String::new();
            }
            if !current_chunk.is_empty() {
                current_chunk.push('\n');
            }
            current_chunk.push_str(&line);
        }

        if !current_chunk.is_empty() {
            ctx.say(current_chunk).await?;
        }
    }

    cache.update_seen_tracks(&user_id, new_seen);
    cache.save("user_cache.json")?;

    Ok(())
}

/// Reset a user's seen records
#[poise::command(slash_command)]
pub async fn reset(ctx: Context<'_>) -> Result<(), Error> {
    let user_id = ctx.author().id.to_string();

    let mut cache = UserCache::load("user_cache.json").unwrap_or_default();

    if cache.get_seen_tracks(&user_id).is_empty() {
        ctx.say("You have no seen records to reset.").await?;
    } else {
        cache.update_seen_tracks(&user_id, HashSet::new());
        cache.save("user_cache.json")?;
        ctx.say("Your seen records have been reset.").await?;
    }

    Ok(())
}

fn parse_time(s: &str) -> Option<f64> {
    let s = s.replace('’', "'").replace(['“', '″'], "\"");

    let parts: Vec<&str> = s.split(['\'', '"']).collect();
    if parts.len() != 3 {
        return None;
    }

    let minutes = parts[0].parse::<f64>().ok()?;
    let seconds = parts[1].parse::<f64>().ok()?;
    let millis = parts[2].parse::<f64>().ok()? / 1000.0;

    Some(minutes * 60.0 + seconds + millis)
}
