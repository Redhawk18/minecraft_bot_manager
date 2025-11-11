use iced::{
    Element, Length,
    widget::{button, center, column, text, text_input, toggler},
};
use prismlauncher::{Address, GameMode, Hostname};
use std::{borrow::Cow, net::IpAddr};
use tracing::trace;

use crate::action::Action;

#[derive(Debug, Clone)]
pub struct Configure {
    pub username: String,
    pub instance_name: String,
    is_multiplayer: bool,
    gamemode_raw: String,
    pub gamemode: Option<prismlauncher::GameMode>,
}

pub enum Instruction {
    Submit,
}

#[derive(Debug, Clone)]
pub enum Message {
    BotNameChange(String),
    InstanceNameChange(String),
    IsMultiplayer(bool),
    GamemodeChange(String),
    Submit,
}

impl Configure {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, message: Message) -> Action<Instruction, Message> {
        match message {
            Message::BotNameChange(name) => self.username = name,
            Message::InstanceNameChange(name) => self.instance_name = name,
            Message::IsMultiplayer(bool) => self.is_multiplayer = bool,
            Message::GamemodeChange(name) => {
                self.gamemode_raw = name.clone();

                self.gamemode = Some(if self.is_multiplayer {
                    match name.parse::<IpAddr>() {
                        Ok(ip_addr) => match ip_addr {
                            IpAddr::V4(ip4) => GameMode::MultiPlayer(Address {
                                hostname: Hostname::Ipv4(ip4),
                                ..Default::default()
                            }),
                            IpAddr::V6(ip6) => GameMode::MultiPlayer(Address {
                                hostname: Hostname::Ipv6(ip6),
                                ..Default::default()
                            }),
                        },
                        Err(_) => {
                            trace!("Fallback method, can't parse IP so we assume its DNS.");
                            GameMode::MultiPlayer(Address {
                                hostname: Hostname::Dns(name),
                                ..Default::default()
                            })
                        }
                    }
                } else {
                    // assume input is just world name
                    GameMode::SinglePlayer { world_name: name }
                });
            }
            Message::Submit => return Action::new(Instruction::Submit),
        }

        Action::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        const SPACE: f32 = 230.0;

        center(
            column![
                text("Bot Name"),
                text_input("mankool69", &self.username)
                    .on_input(Message::BotNameChange)
                    .width(SPACE),
                text("Prismlauncher Instance"),
                text_input("const_mankool69", &self.instance_name)
                    .width(SPACE)
                    .on_input(Message::InstanceNameChange),
                toggler(self.is_multiplayer).on_toggle(Message::IsMultiplayer),
                if self.is_multiplayer {
                    column![
                        text("Multiplayer Server Host"),
                        text_input("Server IP", &self.gamemode_raw)
                            .on_input(Message::GamemodeChange)
                            .width(SPACE)
                    ]
                } else {
                    column![
                        text("Singleplayer World Name"),
                        text_input("World Name", &self.gamemode_raw)
                            .on_input(Message::GamemodeChange)
                            .width(SPACE),
                    ]
                },
                button("Submit").on_press(Message::Submit),
            ]
            .padding(3)
            .spacing(10),
        )
        .into()
    }
}

impl Default for Configure {
    fn default() -> Self {
        Self {
            username: String::new(),
            instance_name: String::new(),
            is_multiplayer: true,
            gamemode: None,
            gamemode_raw: String::new(),
        }
    }
}
