pub mod age;
pub mod mkwrs;
pub mod register;

use crate::Error;
use poise::Command;

pub fn get_commands() -> Vec<Command<crate::Data, Error>> {
    vec![age::age(), register::register(), mkwrs::mkwrs()]
}
