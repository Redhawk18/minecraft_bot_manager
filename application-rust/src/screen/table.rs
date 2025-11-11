use iced::{
    Element,
    widget::{column, table, text},
};
use prismlauncher::GameMode;
use std::borrow::Cow;

use crate::{Bot, action::Action};

pub enum Instruction {}

#[derive(Debug, Clone)]
pub enum Message {}

pub fn update(message: Message) -> Action<Instruction, Message> {
    match message {}

    Action::none()
}

pub fn view(bot_list: &[Bot]) -> Element<Message> {
    let table = {
        let columns = vec![
            table::column(text("Username"), |bot: &Bot| text(&bot.username)),
            table::column(text("Status"), |bot: &Bot| text(bot.status.to_string())),
            table::column(text("Instance"), |bot: &Bot| text(&bot.instance)),
            table::column(text("Server"), |bot: &Bot| {
                text(match &bot.server {
                    // Avoids allocation.
                    GameMode::SinglePlayer { world_name } => Cow::from(world_name),
                    GameMode::MultiPlayer(address) => address.to_string().into(),
                })
            }),
        ];

        table(columns, bot_list)
    };

    column![table,].into()
}
