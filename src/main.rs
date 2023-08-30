mod jikan;

use dotenv::dotenv;
use reqwest::Url;
use teloxide::{
    prelude::*,
    utils::command::BotCommands,
    types::InputFile,
    types::ParseMode,
};
use jikan::{
    JikanResponse,
    Datum,
    ImageExtension,
    Status
};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Start the bot.")]
    Start,
    #[command(description = "Display this help message.")]
    Help,
    #[command(description = "Get anime info.")]
    Info(String),
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => bot.send_message(msg.chat.id, "Hi!, welcome to my bot, use /help to see the commands :D").await?,
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Info(anime_name) => {
            // anime name must be provided
            if anime_name.is_empty() {
                bot.send_message(msg.chat.id, "Please enter anime name.").await?;

                return Ok(());
            }

            let response = match get_anime_info(anime_name).await {
                Ok(response) => {
                    if response.data.is_empty() {
                        bot.send_message(msg.chat.id, "Anime not found")
                            .await?;

                        return Ok(());
                    }

                    response.data[0].clone()
                },
                Err(_) => {
                    bot.send_message(msg.chat.id, "Error getting anime info")
                        .await?;

                    return Ok(());
                },
            };

            let message_template = format_message(&response);
            let img_url = match Url::parse(
                &response.images.get(&ImageExtension::Jpg)
                    .unwrap()
                    .large_image_url
            ) {
                Ok(large_image_url) => large_image_url,
                Err(_) => {
                    bot.send_message(msg.chat.id, "Something went wrong getting the image url")
                        .await?;

                    return Ok(());
                }
            };

            bot.send_photo(msg.chat.id, InputFile::url(img_url))
                .caption(format!("{}", message_template))
                .parse_mode(ParseMode::MarkdownV2)
                .await?
        }
    };

    return Ok(())
}

// get the anime info with reqwest and the name provided
async fn get_anime_info(anime_name: String) -> Result<JikanResponse, reqwest::Error> {
    // Only get the first result
    let api_url = format!("https://api.jikan.moe/v4/anime?q={}&sfw&limit=1", anime_name);
    let response = reqwest::get(&api_url)
        .await?
        .json::<JikanResponse>()
        .await?;

    return Ok(response)
}

fn format_message(data: &Datum) -> String {
    let message = format!(r#"Title: *{}*
English title: *{}*

episodes: *{}*
Aired: *{}*
Status: *{}*"#,
        data.title,
        data.title_english,
        data.episodes,
        data.aired.string,
        match data.status {
            Status::FinishedAiring => "Finished Airing",
            Status::Airing => "Currently Airing",
            Status::NotYetAired => "Not yet aired",
        }
    );

    return message
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let bot = Bot::from_env();

    log::info!("Starting bot...");
    Command::repl(bot, answer).await;
}

