use std::time::Instant;

use iced::{
    Animation, Element, Length, Subscription, Task, animation::Easing, widget::{Container, Row}, window,
};
use sqlx::SqlitePool;
use crate::ui::{components::sidebar, screens::{cipher::{self, CipherState}, home::{self, HomeState}}};
use crate::ui::screens::fence::{self, FenceState};

#[derive(Debug, Clone)]
pub enum Message {
    Navigate(Screen),
    Fence(fence::FenceMessage),
    Cipher(cipher::CipherMessage),
    Home(home::HomeMessage),
    ToggleSidebar,
    Tick(Instant),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Screen {
    Fence,
    Cipher,
    Home,
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
    home: HomeState,
    sidebar: Animation<bool>,
    now: Instant,
}

impl App {
    pub fn new(pool: SqlitePool) -> (Self, Task<Message>) {
        (
            Self {
                db: pool,
                screen: Screen::Cipher,
                fence: FenceState::default(),
                cipher: CipherState::default(),
                home: HomeState::default(),
                sidebar: Animation::new(true).easing(Easing::EaseInOut).quick(),
                now: Instant::now(),
            },
            Task::none(),
        )
    }

pub fn view(&self) -> Element<'_, Message> {
    let width = self.sidebar.interpolate(sidebar::COLLAPSED_WIDTH, sidebar::EXPANDED_WIDTH, self.now);

    let expanded = self.sidebar.value();

    let sidebar = sidebar::view(width, expanded, self.screen);

    let screen_view = match self.screen {
        Screen::Fence => self.fence.view().map(Message::Fence),
        Screen::Cipher => self.cipher.view().map(Message::Cipher),
        Screen::Home => self.home.view().map(Message::Home),
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
            Message::Home(msg) => self.home.update(msg),
            Message::ToggleSidebar => {
                let now = Instant::now();
                let target = self.sidebar.value();
                self.sidebar.go_mut(!target, now);
                self.now = now;
            }
            Message::Tick(now) => self.now = now,
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        if self.sidebar.is_animating(Instant::now()) {
            window::frames().map(|_| Message::Tick(Instant::now()))
        } else {
            Subscription::none()
        }
    }
}
