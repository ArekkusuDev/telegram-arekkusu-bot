mod jikan;

use dotenv::dotenv;
use teloxide::{
    prelude::*,
    utils::command::BotCommands,
};
use jikan::{
    JikanResponse,
    Datum,
};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Display this message.")]
    Help,
    #[command(description = "Get anime info.")]
    Info(String),
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Info(anime_name) => {
            // anime name must be provided
            if anime_name.is_empty() {
                bot.send_message(msg.chat.id, "Please enter anime name.").await?;
                return Ok(());
            }

            let response = match get_anime_info(anime_name).await {
                Ok(response) => response.data[0].clone(),
                Err(_) => {
                    bot.send_message(msg.chat.id, "This anime doesn't exist").await?;
                    return Ok(());
                }
            };

            let message_template = format_message(response).await;
            bot.send_message(msg.chat.id, format!("{message_template}")).await?
        }
    };

    Ok(())
}

// get the anime info with reqwest and the name provided
async fn get_anime_info(anime_name: String) -> Result<JikanResponse, reqwest::Error> {
    let url = format!("https://api.jikan.moe/v4/anime?q={}&sfw&limit=1", anime_name);
    let resp = reqwest::get(&url).await?.json::<JikanResponse>().await?;

    Ok(resp)
}

async fn format_message(data: Datum) -> String {
    let mut message = String::new();

    message.push_str(&format!("Title: {}\nEnglish: {}\nJapanese: {}", data.title, data.title_english, data.title_japanese));

    message
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let bot = Bot::from_env();

    log::info!("Starting bot...");
    Command::repl(bot, answer).await;
}

