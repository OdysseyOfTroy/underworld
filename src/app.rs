use iced::{widget::{Button, Column, Row}, Element, Task};

use crate::ui::{cipher::{self, CipherState}, fence::{self, FenceState}};

#[derive(Debug, Clone)]
pub enum Message {
    NavigateToFence,
    NavigateToCipher,
    Fence(fence::Message),
    Cipher(cipher::Message),
}

enum Screen {
    Fence,
    Cipher,
}

pub struct App {
    screen: Screen,
    fence: FenceState,
    cipher: CipherState,
}

impl App {
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
            .push(Button::new("Fence").on_press(Message::NavigateToFence))
            .push(Button::new("Cipher").on_press(Message::NavigateToCipher));

        let content = match self.screen {
            Screen::Fence => self.fence.view().map(Message::Fence),
            Screen::Cipher => self.cipher.view().map(Message::Cipher),
        };

        Column::new()
            .spacing(20)
            .push(nav)
            .push(content)
            .into()
    }

    pub fn update(&mut self, message: Message) {
match message {
            Message::NavigateToFence => self.screen = Screen::Fence,
            Message::NavigateToCipher => self.screen = Screen::Cipher,

            Message::Fence(msg) => self.fence.update(msg),
            Message::Cipher(msg) => self.cipher.update(msg),
        }
    }
}
