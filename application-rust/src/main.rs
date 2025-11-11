mod action;
mod screen;

use iced::{Element, Task, exit};
use std::{fmt::Display, mem};

use screen::configure::Configure;

pub fn main() -> iced::Result {
    iced::application(Application::new, Application::update, Application::view).run()
}

#[derive(Default)]
struct Application {
    bots: Vec<Bot>,

    screen: Screen,
    state: State,
}

enum Instruction {
    /// The setup page to configure and add a new bot.
    Configure(screen::configure::Instruction),
    /// The table view screen with all bot's in table form.
    Table(screen::table::Instruction),
}

/// What module we use to render the entire UI. This makes our code greatly simplified.
#[derive(Debug, Clone, Copy, Default)]
enum Screen {
    /// The setup page to configure and add a new bot.
    Configure,
    #[default]
    /// The table view screen with all bot's in table form.
    Table,
}

/// The state of each screen so we can abstract who will create state
#[derive(Default)]
struct State {
    configure: Configure,
}

/// A single bot, can only connect to 1 server through 1 instance and do its 1 script.
#[derive(Debug, Clone)]
struct Bot {
    username: String,
    status: Status,
    instance: Instance,
    server: prismlauncher::GameMode,
    game_infomation: Option<GameInfomation>,
}

#[derive(Debug, Clone, Copy)]
struct Coordinates {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}

#[derive(Debug, Clone, Copy)]
enum Dimension {
    /// The Overworld.
    Overworld,
    /// The Nether.
    Nether,
    /// The End.
    End,
    // todo for modded
    // Other { dimension_name: String },
}

#[derive(Debug, Clone, Copy)]
struct GameInfomation {
    coordinates: Coordinates,
    dimension: Dimension,
    memory_usage: usize,
    // game_handle
}

impl Bot {
    pub fn new(username: String, instance_name: String, server: prismlauncher::GameMode) -> Self {
        Self {
            username,
            status: Status::Offline,
            instance: instance_name,
            server,
            game_infomation: None,
        }
    }
}

type Instance = String;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    #[default]
    Offline,
    Online,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Status::Offline => "Offline",
                Status::Online => "Online",
            }
        )
    }
}

#[derive(Debug, Clone)]
enum Message {
    Exit,
    Screen(Screen),

    Configure(screen::configure::Message),
    Table(screen::table::Message),
}

impl Application {
    fn new() -> Self {
        Self {
            bots: vec![],
            screen: Screen::Configure,
            state: State::default(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Exit => exit(),
            Message::Screen(new_screen) => {
                self.screen = new_screen;
                Task::none()
            }
            Message::Configure(message) => {
                let action = self.state.configure.update(message);
                if let Some(instruction) = action.instruction {
                    self.perform(Instruction::Configure(instruction));
                }

                action.task.map(Message::Configure)
            }
            Message::Table(message) => {
                screen::table::update(message);
                Task::none()
            }
        }
    }

    // Performs [`Instructions`] that descendants call to mutate [`Self`]'s state.
    fn perform(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Configure(instruction) => match instruction {
                screen::configure::Instruction::Submit => {
                    // Just take what we want.
                    let username = mem::take(&mut self.state.configure.username);
                    let instance_name = mem::take(&mut self.state.configure.instance_name);
                    let gamemode =
                        mem::take(&mut self.state.configure.gamemode).expect("Not initialized");

                    let bot = Bot::new(username, instance_name, gamemode);
                    self.bots.push(bot);

                    self.screen = Screen::Table;
                }
            },
            Instruction::Table(instruction) => todo!(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let element = match &self.screen {
            Screen::Configure => self.state.configure.view().map(Message::Configure),
            Screen::Table => screen::table::view(&self.bots).map(Message::Table),
        };

        element.explain(iced::Color::BLACK)
    }
}
