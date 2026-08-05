use iced::{
    Element, Task, Length,
    widget::{Row, Container},
};
use sqlx::SqlitePool;
use crate::ui::{components::sidebar, screens::cipher::{self, CipherState}};
use crate::ui::screens::fence::{self, FenceState};

#[derive(Debug, Clone)]
pub enum Message {
    Navigate(Screen),
    Fence(fence::FenceMessage),
    Cipher(cipher::CipherMessage),
    ToggleSidebar,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub db: SqlitePool,
    screen: Screen,
    fence: FenceState,
    cipher: CipherState,
    sidebar_collapsed: bool,
}

impl App {
    pub fn title(&self) -> String {
        match self.screen {
            Screen::Fence => "Fence Calculator".into(),
            Screen::Cipher => "Cipher Tool".into(),
        }
    }

    pub fn new(pool: SqlitePool) -> (Self, Task<Message>) {
        (
            Self {
                db: pool,
                screen: Screen::Cipher,
                fence: FenceState::default(),
                cipher: CipherState::default(),
                sidebar_collapsed: false,
            },
            Task::none(),
        )
    }

pub fn view(&self) -> Element<'_, Message> {
    let sidebar = sidebar::view(self.sidebar_collapsed, self.screen);

    let screen_view = match self.screen {
        Screen::Fence => self.fence.view().map(Message::Fence),
        Screen::Cipher => self.cipher.view().map(Message::Cipher),
    };

    let main_content = Container::new(screen_view)
        .padding(20)
        .width(Length::Fill);

    Row::new()
        .push(sidebar)
        .push(main_content)
        .into()
}

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Navigate(screen) => self.screen = screen,
            Message::Fence(msg) => self.fence.update(msg),
            Message::Cipher(msg) => self.cipher.update(msg),
            Message::ToggleSidebar => self.sidebar_collapsed = !self.sidebar_collapsed,
        }
    }
}
