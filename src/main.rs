mod zone;
mod dialogue;
use std::error::Error;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{
        InlineKeyboardButton, InlineKeyboardMarkup, InlineQueryResultArticle, InputMessageContent,
        InputMessageContentText, Me,
    },
    utils::command::BotCommands,
};



/// These commands are supported:
#[derive(BotCommands)]
#[command(rename_rule = "lowercase")]
enum Command {
    /// Display this text
    Help,
    /// Start
    #[command(description = "start")]
    Start,

    #[command(description = "test")]
    Get,

    #[command(description = "add")]
    Add
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //pretty_env_logger::init();
    log::info!("Starting buttons bot...");

    let bot = Bot::from_env();

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler))
        .branch(Update::filter_inline_query().endpoint(inline_query_handler));

    Dispatcher::builder(bot, handler).enable_ctrlc_handler().build().dispatch().await;
    Ok(())
}

/// Creates a keyboard made by buttons in a big column.
fn make_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let mut uchastki1 = zone::Uchastki
    {
        uchastki: vec![]
    };
    let uchastki = uchastki1.get_uchastki();

    for uchastok in uchastki.chunks(3) {
        let row = uchastok
            .iter()
            .map(|uchastok| InlineKeyboardButton::callback(uchastok.name.to_owned(), uchastok.name.to_owned()))
            .collect();

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

/// Parse the text wrote on Telegram and check if that text is a valid command
/// or not, then match the command. If the command is `/start` it writes a
/// markup with the `InlineKeyboardMarkup`.
async fn message_handler(
    bot: Bot,
    msg: Message,
    me: Me,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        match BotCommands::parse(text, me.username()) {
            Ok(Command::Help) => {
                // Just send the description of all commands.
                bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
            }
            Ok(Command::Start) => {
                // Create a list of buttons and send them.
                let keyboard = make_keyboard();
                bot.send_message(msg.chat.id, "Выберите участок:").reply_markup(keyboard).await?;
            }

            Ok(Command::Get) => {
                let mut ouch = zone::Uchastki{
                    uchastki: vec![]
                };
                let mut uch = ouch.get_uchastki();
                for i in 0..uch.len() {
                    bot.send_message(msg.chat.id, uch[i].name.to_string()).await?;
                }
            }

            Ok(Command::Add) => {
                let mut ouch = zone::Uchastki{
                    uchastki: vec![]
                };
                
                let y: zone::Zone = zone::Zone {
                    name: "бутово",
                    observables: "нужно удобрить",
                    infrastructure: "трудно подъехать",
                    commentary: "все плохо"
                };
                
                let x: zone::Uchastok = zone::Uchastok {
                    zone: y,
                    name: "бутово",
                    description: "полная залупа"
                };

                let mut uch = ouch.add_uchastki(x);
                
                bot.send_message(msg.chat.id, "Успешно добавлен участок ".to_string() + x.name).await?;
            }

            Err(_) => {
                bot.send_message(msg.chat.id, "Command not found!").await?;
            }
        }
    }

    Ok(())
}

async fn inline_query_handler(
    bot: Bot,
    q: InlineQuery,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let choose_debian_version = InlineQueryResultArticle::new(
        "0",
        "Выберите участок:",
        InputMessageContent::Text(InputMessageContentText::new("Участки:")),
    )
    .reply_markup(make_keyboard());

    bot.answer_inline_query(q.id, vec![choose_debian_version.into()]).await?;

    Ok(())
}

/// When it receives a callback from a button it edits the message with all
/// those buttons writing a text with the selected Debian version.
///
/// **IMPORTANT**: do not send privacy-sensitive data this way!!!
/// Anyone can read data stored in the callback button.
async fn callback_handler(bot: Bot, q: CallbackQuery) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(version) = q.data {
        let text = format!("Участок: {version}");

        // Tell telegram that we've seen this query, to remove 🕑 icons from the
        // clients. You could also use `answer_callback_query`'s optional
        // parameters to tweak what happens on the client side.
        bot.answer_callback_query(q.id).await?;

        // Edit text of the message to which the buttons were attached
        if let Some(Message { id, chat, .. }) = q.message {
            bot.edit_message_text(chat.id, id, text).await?;
        } else if let Some(id) = q.inline_message_id {
            bot.edit_message_text_inline(id, text).await?;
        }

        log::info!("Выбрано: {}", version);
    }

    Ok(())
}