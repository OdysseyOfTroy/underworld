use iced::{widget::{Button, Column, Row}, Element, Task};

use crate::ui::{cipher::{self, CipherState}, fence::{self, FenceState}};

#[derive(Debug, Clone)]
pub enum Message {
    Navigate(Screen),
    Fence(fence::FenceMessage),
    Cipher(cipher::CipherMessage),
}

#[derive(Debug, Clone)]
pub enum Screen {
    Fence,
    Cipher,
}

pub trait AppScreen {
    type Msg;

    fn update(&mut self, msg: Self::Msg);
    fn view(&self) -> Element<'_, Self::Msg>;
}

pub struct App {
    screen: Screen,
    fence: FenceState,
    cipher: CipherState,
}

impl App {
    pub fn title(&self) -> String {
        match self.screen {
            Screen::Fence => "Fence Calculator".into(),
            Screen::Cipher => "Cipher Tool".into(),
        }
    }

   pub fn new() -> (Self, Task<Message>) {
        (
        Self {
            screen: Screen::Cipher,
            fence: FenceState::default(),
            cipher: CipherState::default(),
        },
        Task::none(),
        )
    }

   pub fn view(&self) -> Element<'_, Message> {
        let nav = Row::new()
            .spacing(20)
            .push(Button::new("Fence").on_press(Message::Navigate(Screen::Fence)))
            .push(Button::new("Cipher").on_press(Message::Navigate(Screen::Cipher)));

        let screen_view = match self.screen {
            Screen::Fence => self.fence.view().map(Message::Fence),
            Screen::Cipher => self.cipher.view().map(Message::Cipher),
        };

        Column::new()
            .spacing(20)
            .push(nav)
            .push(screen_view)
            .into()
    }

    pub fn update(&mut self, message: Message) {
match message {
            Message::Navigate(screen) => self.screen = screen,
            Message::Fence(msg) => self.fence.update(msg),
            Message::Cipher(msg) => self.cipher.update(msg),
        }
    }
}
