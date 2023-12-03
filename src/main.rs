use teloxide::{prelude::*, utils::command::BotCommands};
use std::fs::*;
use std::io::Write;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
    #[command(description = "test")]
    Test,
    #[command(description = "start interaction")]
    Start,
    #[command(description = "take user data")]
    Login(String)
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Username(username) => {
            bot.send_message(msg.chat.id, format!("Your username is @{username}.")).await?
        }
        Command::UsernameAndAge { username, age } => {
            bot.send_message(msg.chat.id, format!("Your username is @{username} and age is {age}."))
                .await?
        }
        Command::Test => bot.send_message(msg.chat.id, format!("test")).await?,
        Command::Start => bot.send_message(msg.chat.id, format!("Здравствуйте, добро пожаловать в бота озеленения и ко, пожалуйста вызовите команду /login чтобы я смог Вас  запомнить")).await?,
        Command::Login(login) => {
            let mut data = File::create("src/data.txt")?;
            data.write(login.as_bytes())?;
            bot.send_message(msg.chat.id, format!("done")).await?
        }
    };

    Ok(())
}